#![deny(unsafe_code)]
use futures::{
    executor::{block_on, ThreadPool},
    future::{abortable, join_all},
    future::{AbortHandle, Either},
    task::{Spawn, SpawnExt},
    FutureExt, StreamExt,
};
use futures_timer::Delay;
use progress_dashboard::tui::{ticker, Event};
use progress_dashboard::{tui, Tree, TreeKey, TreeRoot};
use rand::prelude::*;
use std::{error::Error, future::Future, ops::Add, time::Duration, time::SystemTime};

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

async fn work_forever(pool: impl Spawn + Clone + Send + 'static) -> Result {
    let progress = progress_dashboard::TreeRoot::new();
    // Now we should handle signals to be able to cleanup properly
    let (gui_handle, abort_gui) = launch_ambient_gui(&pool, progress.clone()).unwrap();
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
) -> std::result::Result<(impl Future<Output = ()>, AbortHandle), std::io::Error> {
    let mut offset_xy = (0u16, 0u16);
    let mut direction = Direction::Shrink;
    let render_fut = tui::render_with_input(
        progress,
        tui::Config {
            title: TITLES.choose(&mut thread_rng()).map(|t| *t).unwrap().into(),
            frames_per_second: 10.0,
        },
        ticker(Duration::from_millis(30)).map(move |_| {
            let (width, height) = termion::terminal_size().unwrap_or((30, 30));
            let (ref mut ofs_x, ref mut ofs_y) = offset_xy;
            let min_size = 2;
            match direction {
                Direction::Shrink => {
                    *ofs_x = ofs_x.saturating_add(1);
                    *ofs_y = ofs_y.saturating_add(1);
                }
                Direction::Grow => {
                    *ofs_x = ofs_x.saturating_sub(1);
                    *ofs_y = ofs_y.saturating_sub(1);
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
        }),
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

fn main() -> Result {
    env_logger::init();
    // Use spawn as well to simulate Send futures
    let pool = ThreadPool::builder()
        .pool_size(1)
        .create()
        .expect("pool creation to work (io-error is not Send");
    block_on(work_forever(pool))
}

struct NestingLevel(u8);
type Result = std::result::Result<(), Box<dyn Error + Send>>;
