use futures::future::{AbortHandle, Either};
use futures::{
    executor::{block_on, ThreadPool},
    future::{abortable, join},
    task::{Spawn, SpawnExt},
    FutureExt,
};
use futures_timer::Delay;
use progress_dashboard::{tui, Tree, TreeRoot};
use rand::prelude::*;
use std::future::Future;
use std::{error::Error, time::Duration};

const MAX_STEPS: u8 = 100;
const UNITS: &[&str] = &["Mb", "kb", "items", "files"];
const WORK_DELAY_MS: u64 = 100;
const SPAWN_DELAY_MS: u64 = 500;

async fn work_item(mut progress: Tree) -> () {
    let max: u8 = random();
    progress.init(
        if max > MAX_STEPS {
            None
        } else {
            Some((max % MAX_STEPS).into())
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

async fn find_work(prefix: &str, max: NestingLevel, tree: TreeRoot, pool: impl Spawn) -> Result {
    let NestingLevel(max_level) = max;
    let mut level_progress = tree.add_child(format!("{}: Level {}", prefix, 1));
    for level in 0..max_level {
        // one-off ambient tasks
        level_progress.init(Some(max_level as u32), Some("work items"));
        let num_tasks = max_level as usize * 2;
        for id in 0..num_tasks {
            pool.spawn(work_item(
                level_progress.add_child(format!("work {}", id + 1)),
            ))
            .expect("spawn to work");
            level_progress.set(id as u32);
            Delay::new(Duration::from_millis(SPAWN_DELAY_MS)).await;
        }
        level_progress = level_progress.add_child(format!("Level {}", level + 1));
    }

    Ok(())
}

async fn work_forever(pool: impl Spawn + Clone + Send + 'static) -> Result {
    let progress = progress_dashboard::TreeRoot::new();
    // Now we should handle signals to be able to cleanup properly
    let (gui_handle, abort_gui) = launch_ambient_gui(&pool, progress.clone()).unwrap();
    let mut gui_handle = Some(gui_handle.boxed());

    for _ in 0..1 {
        let local_work = find_work("local", NestingLevel(2), progress.clone(), pool.clone());
        let threaded_work = pool
            .spawn_with_handle(find_work(
                "pooled",
                NestingLevel(4),
                progress.clone(),
                pool.clone(),
            ))
            .expect("spawning to work - SpawnError cannot be ");

        match futures::future::select(
            join(local_work.boxed_local(), threaded_work),
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
