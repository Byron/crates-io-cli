use futures::future::{AbortHandle, Either};
use futures::{
    executor::{block_on, ThreadPool},
    future::{abortable, join_all},
    task::{Spawn, SpawnExt},
    FutureExt,
};
use futures_timer::Delay;
use progress_dashboard::{tui, Key, Tree, TreeRoot};
use rand::prelude::*;
use std::future::Future;
use std::{error::Error, time::Duration};

const WORK_STEPS_NEEDED_FOR_UNBOUNDED_TASK: u8 = 100;
const UNITS: &[&str] = &["Mb", "kb", "items", "files"];
const WORK_DELAY_MS: u64 = 100;
const SPAWN_DELAY_MS: u64 = 200;

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
        Delay::new(Duration::from_millis(WORK_DELAY_MS)).await;
    }
    ()
}

async fn find_work(
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
        level_progress.init(Some(num_tasks as u32), Some("work items"));
        for id in 0..num_tasks {
            let handle = pool
                .spawn_with_handle(work_item(
                    level_progress.add_child(format!("work {}", id + 1)),
                ))
                .expect("spawn to work");
            handles.push(handle);
            level_progress.set(id as u32 + 1);

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
        let local_work = find_work(
            format!("{}: local", iteration),
            NestingLevel(thread_rng().gen_range(0, Key::max_level())),
            progress.clone(),
            pool.clone(),
        );
        let pooled_work = (0..thread_rng().gen_range(3, 8usize)).map(|_| {
            pool.spawn_with_handle(find_work(
                format!("{}: pooled", iteration),
                NestingLevel(thread_rng().gen_range(0, Key::max_level())),
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

fn launch_ambient_gui(
    pool: &dyn Spawn,
    progress: TreeRoot,
) -> std::result::Result<(impl Future<Output = ()>, AbortHandle), std::io::Error> {
    let render_fut = tui::render(
        progress,
        tui::Config {
            frames_per_second: 30,
        },
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
