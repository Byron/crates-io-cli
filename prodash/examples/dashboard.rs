#![deny(unsafe_code)]
use futures::{
    executor::{block_on, ThreadPool},
    future::{abortable, join_all},
    future::{AbortHandle, Either},
    task::{Spawn, SpawnExt},
    Future, FutureExt, StreamExt,
};
use futures_timer::Delay;
use prodash::tui::Line;
use prodash::{
    tui,
    tui::{ticker, Event},
    Tree, TreeKey, TreeRoot,
};
use rand::prelude::*;
use std::{error::Error, ops::Add, time::Duration, time::SystemTime};

const WORK_STEPS_NEEDED_FOR_UNBOUNDED_TASK: u8 = 100;
const UNITS: &[&str] = &["Mb", "kb", "items", "files"];
const TITLES: &[&str] = &[" Dashboard Demo ", " 仪表板演示 "];
const WORK_NAMES: &[&str] = &[
    "Downloading Crate",
    "下载板条箱",
    "Running 'cargo geiger'",
    "运行程序 'cargo geiger'",
    "Counting lines of code",
    "计数代码行",
    "Checking for unused dependencies",
    "检查未使用的依赖项",
    "Checking for crate-bloat",
    "检查板条箱膨胀",
    "Generating report",
    "生成报告",
];
const DONE_MESSAGES: &[&str] = &[
    "Yeeeehaa! Finally!!",
    "呀！ 最后！",
    "It feels good to be done!",
    "感觉好极了！",
    "Told you so!!",
    "告诉过你了！",
];
const FAIL_MESSAGES: &[&str] = &[
    "That didn't seem to work!",
    "那似乎没有用！",
    "Oh my… I failed you 😞",
    "哦，我…我让你失败😞",
    "This didn't end well…",
    "结局不好…",
];
const INFO_MESSAGES: &[&str] = &[
    "Making good progress!",
    "进展良好！",
    "Humming along…",
    "嗡嗡作响…",
    "It will be done soooooon…",
    "会很快完成的……",
];
const WORK_DELAY_MS: u64 = 100;
const LONG_WORK_DELAY_MS: u64 = 2000;
const SPAWN_DELAY_MS: u64 = 200;
const CHANCE_TO_BLOCK_PER_STEP: f64 = 1.0 / 100.0;
const CHANCE_TO_SHOW_ETA: f64 = 0.5;

async fn work_item(mut progress: Tree) -> () {
    let max: u8 = thread_rng().gen_range(25, 125);
    progress.init(
        if max > WORK_STEPS_NEEDED_FOR_UNBOUNDED_TASK {
            None
        } else {
            Some(max.into())
        },
        if (max as usize % UNITS.len() + 1) == 0 {
            None
        } else {
            UNITS.choose(&mut thread_rng()).map(|&s| s)
        },
    );

    for step in 0..max {
        progress.set(step as u32);
        let delay = if thread_rng().gen_bool(CHANCE_TO_BLOCK_PER_STEP) {
            let eta = if thread_rng().gen_bool(CHANCE_TO_SHOW_ETA) {
                Some(SystemTime::now().add(Duration::from_millis(LONG_WORK_DELAY_MS)))
            } else {
                None
            };
            progress.blocked(eta);
            thread_rng().gen_range(WORK_DELAY_MS, LONG_WORK_DELAY_MS)
        } else {
            WORK_DELAY_MS
        };
        if thread_rng().gen_bool(0.01) {
            progress.info(INFO_MESSAGES.choose(&mut thread_rng()).unwrap());
        }
        if thread_rng().gen_bool(0.01) {
            progress.set_name(WORK_NAMES.choose(&mut thread_rng()).unwrap().to_string());
        }
        Delay::new(Duration::from_millis(delay)).await;
    }
    if thread_rng().gen_bool(0.95) {
        progress.done(DONE_MESSAGES.choose(&mut thread_rng()).unwrap());
    } else {
        progress.fail(FAIL_MESSAGES.choose(&mut thread_rng()).unwrap());
    }
}

async fn new_chunk_of_work(
    prefix: impl AsRef<str>,
    max: NestingLevel,
    tree: TreeRoot,
    pool: impl Spawn,
) -> Result {
    let NestingLevel(max_level) = max;
    let mut progresses = Vec::new();
    let mut level_progress =
        tree.add_child(format!("{}: level {} of {}", prefix.as_ref(), 1, max_level));
    let mut handles = Vec::new();

    for level in 0..max_level {
        // one-off ambient tasks
        let num_tasks = max_level as usize * 2;
        for id in 0..num_tasks {
            let handle = pool
                .spawn_with_handle(work_item(level_progress.add_child(format!(
                    "{} {}",
                    WORK_NAMES.choose(&mut thread_rng()).unwrap(),
                    id + 1
                ))))
                .expect("spawn to work");
            handles.push(handle);

            Delay::new(Duration::from_millis(SPAWN_DELAY_MS)).await;
        }
        if level + 1 != max_level {
            let tmp = level_progress.add_child(format!("Level {}", level + 1));
            progresses.push(level_progress);
            level_progress = tmp;
        }
    }

    progresses.push(level_progress);
    for handle in handles.into_iter() {
        handle.await;
    }

    Ok(())
}

async fn work_forever(pool: impl Spawn + Clone + Send + 'static, args: arg::Options) -> Result {
    let progress = prodash::Config {
        message_buffer_capacity: args.message_scrollback_buffer_size,
        ..prodash::Config::default()
    }
    .create();
    // Now we should handle signals to be able to cleanup properly
    let (gui_handle, abort_gui) = launch_ambient_gui(&pool, progress.clone(), args).unwrap();
    let mut gui_handle = Some(gui_handle.boxed());
    let mut iteration = 0;

    loop {
        iteration += 1;
        let local_work = new_chunk_of_work(
            format!("{}: local", iteration),
            NestingLevel(thread_rng().gen_range(0, TreeKey::max_level())),
            progress.clone(),
            pool.clone(),
        );
        let pooled_work = (0..thread_rng().gen_range(6, 16usize)).map(|_| {
            pool.spawn_with_handle(new_chunk_of_work(
                format!("{}: pooled", iteration),
                NestingLevel(thread_rng().gen_range(0, TreeKey::max_level())),
                progress.clone(),
                pool.clone(),
            ))
            .expect("spawning to work - SpawnError cannot be ")
            .boxed_local()
        });

        match futures::future::select(
            join_all(std::iter::once(local_work.boxed_local()).chain(pooled_work)),
            gui_handle.take().expect("gui handle"),
        )
        .await
        {
            Either::Left((_workblock_result, running_gui)) => {
                gui_handle = Some(running_gui);
                continue;
            }
            Either::Right(_gui_shutdown) => break,
        }
    }

    abort_gui.abort();
    if let Some(gui) = gui_handle {
        gui.await;
    }
    Ok(())
}

enum Direction {
    Shrink,
    Grow,
}

fn launch_ambient_gui(
    pool: &dyn Spawn,
    progress: TreeRoot,
    args: arg::Options,
) -> std::result::Result<(impl Future<Output = ()>, AbortHandle), std::io::Error> {
    let render_fut = tui::render_with_input(
        progress,
        tui::Config {
            title: TITLES.choose(&mut thread_rng()).map(|t| *t).unwrap().into(),
            frames_per_second: args.fps,
        },
        futures::stream::select(
            window_resize_stream(args.animate_terminal_size),
            ticker(Duration::from_millis(1000)).map(|_| {
                if thread_rng().gen_bool(0.5) {
                    Event::SetTitle(TITLES.choose(&mut thread_rng()).unwrap().to_string())
                } else {
                    Event::SetInformation(generate_statistics())
                }
            }),
        ),
    )?;
    let (render_fut, abort_handle) = abortable(render_fut);
    let handle = pool
        .spawn_with_handle(render_fut)
        .expect("GUI to be spawned");
    Ok((
        async move {
            handle.await.ok();
            ()
        },
        abort_handle,
    ))
}

fn generate_statistics() -> Vec<Line> {
    vec![
        Line::Title("Hello World".into()),
        Line::Text("You can put here what you want".into()),
        Line::Text("as long as it fits one line".into()),
        Line::Text("until a certain limit is reached".into()),
        Line::Text("which is when truncation happens".into()),
        Line::Text("这是中文的一些文字。".into()),
        Line::Text("鹅、鹅、鹅 曲项向天歌 白毛浮绿水 红掌拨清波".into()),
        Line::Text("床前明月光, 疑是地上霜。举头望明月，低头思故乡。".into()),
        Line::Text("锄禾日当午，汗滴禾下土。谁知盘中餐，粒粒皆辛苦。".into()),
        Line::Text("春眠不觉晓，处处闻啼鸟。夜来风雨声，花落知多少".into()),
        Line::Text("煮豆燃豆萁，豆在釜中泣。本自同根生，相煎何太急".into()),
        Line::Text(
            "and this line is without any doubt very very long and it really doesn't want to stop"
                .into(),
        ),
        Line::Title("Statistics".into()),
        Line::Text(format!(
            "lines of unsafe code: {}",
            thread_rng().gen_range(0usize, 1000000)
        )),
        Line::Text(format!(
            "wasted space in crates: {} Kb",
            thread_rng().gen_range(100usize, 1000000)
        )),
        Line::Text(format!(
            "unused dependencies: {} crates",
            thread_rng().gen_range(100usize, 1000)
        )),
        Line::Text(format!(
            "average #dependencies: {} crates",
            thread_rng().gen_range(0usize, 500)
        )),
        Line::Text(format!(
            "bloat in code: {} Kb",
            thread_rng().gen_range(100usize, 5000)
        )),
    ]
}

fn window_resize_stream(animate: bool) -> impl futures::Stream<Item = Event> {
    let mut offset_xy = (0u16, 0u16);
    let mut direction = Direction::Shrink;
    if !animate {
        return futures::stream::pending().boxed();
    }

    ticker(Duration::from_millis(100))
        .map(move |_| {
            let (width, height) = termion::terminal_size().unwrap_or((30, 30));
            let (ref mut ofs_x, ref mut ofs_y) = offset_xy;
            let min_size = 2;
            match direction {
                Direction::Shrink => {
                    *ofs_x = ofs_x
                        .saturating_add((1 as f32 * (width as f32 / height as f32)).ceil() as u16);
                    *ofs_y = ofs_y
                        .saturating_add((1 as f32 * (height as f32 / width as f32)).ceil() as u16);
                }
                Direction::Grow => {
                    *ofs_x = ofs_x
                        .saturating_sub((1 as f32 * (width as f32 / height as f32)).ceil() as u16);
                    *ofs_y = ofs_y
                        .saturating_sub((1 as f32 * (height as f32 / width as f32)).ceil() as u16);
                }
            }
            let bound = tui::tui_export::layout::Rect {
                x: 0,
                y: 0,
                width: width.saturating_sub(*ofs_x).max(min_size),
                height: height.saturating_sub(*ofs_y).max(min_size),
            };
            if bound.area() <= min_size * min_size || bound.area() == width * height {
                direction = match direction {
                    Direction::Grow => Direction::Shrink,
                    Direction::Shrink => Direction::Grow,
                };
            }
            Event::SetWindowSize(bound)
        })
        .boxed()
}

fn main() -> Result {
    env_logger::init();

    let args: arg::Options = argh::from_env();
    // Use spawn as well to simulate Send futures
    let pool = ThreadPool::builder()
        .pool_size(1)
        .create()
        .expect("pool creation to work (io-error is not Send");
    block_on(work_forever(pool, args))
}

struct NestingLevel(u8);
type Result = std::result::Result<(), Box<dyn Error + Send>>;

mod arg {
    use argh::FromArgs;

    #[derive(FromArgs)]
    /// Reach new heights.
    pub struct Options {
        /// if set, the terminal window will be animated to assure resizing works as expected.
        #[argh(switch, short = 'a')]
        pub animate_terminal_size: bool,

        /// the amount of frames to show per second, can be below zero, e.g.
        /// 0.25 shows a frame every 4 seconds.
        #[argh(option, default = "10.0")]
        pub fps: f32,

        /// the amount of scrollback for task messages.
        #[argh(option, default = "80")]
        pub message_scrollback_buffer_size: usize,
    }
}
