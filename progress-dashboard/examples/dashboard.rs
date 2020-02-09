use futures::future::{abortable, join, AbortHandle};
use futures::task::{Spawn, SpawnExt};
use futures_timer::Delay;
use log::info;
use progress_dashboard::{tui, TreeRoot};
use rand::prelude::*;
use signal_hook::iterator::Signals;
use std::{error::Error, time::Duration};

const MAX_STEPS: u8 = 100;
const UNITS: &[&str] = &["Mb", "kb", "items", "files"];
const WORK_DELAY_MS: u64 = 50;
const SPAWN_DELAY_MS: u64 = 150;

async fn work_item(mut progress: TreeRoot) -> () {
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
        info!("work-item: wait");
        Delay::new(Duration::from_millis(WORK_DELAY_MS)).await;
    }
    ()
}

async fn find_work(max: NestingLevel, mut tree: TreeRoot, pool: impl Spawn) -> Result {
    let NestingLevel(max_level) = max;
    for level in 0..max_level {
        // one-off ambient tasks
        tree.init(Some(max_level as u32), Some("work items"));
        for id in 0..max_level as usize * 2 {
            pool.spawn(work_item(tree.add_child(format!("work {}", id + 1))))
                .expect("spawn to work");
            info!("spawn work: wait");
            tree.set(id as u32);
            Delay::new(Duration::from_millis(SPAWN_DELAY_MS)).await;
        }
        tree = tree.add_child(format!("Level {}", level + 1));
    }

    Ok(())
}

async fn work_forever(pool: impl Spawn + Clone + Send + 'static) -> Result {
    let progress = progress_dashboard::TreeRoot::new();
    // Now we should handle signals to be able to cleanup properly
    let trigger = launch_ambient_gui(&pool, &progress);
    abort_gui_on_signal(trigger.clone());

    for _ in 1..10 {
        let local_work = find_work(NestingLevel(2), progress.clone(), pool.clone());
        let threaded_work = pool
            .spawn_with_handle(find_work(NestingLevel(2), progress.clone(), pool.clone()))
            .expect("spawning to work - SpawnError cannot be ");

        let _ = join(local_work, threaded_work).await;
    }
    trigger.abort();
    Ok(())
}

fn launch_ambient_gui(pool: &dyn Spawn, progress: &TreeRoot) -> AbortHandle {
    let (abortable_render, trigger) = abortable(tui::render(
        progress.clone(),
        tui::Config {
            frames_per_second: 30,
        },
    ));
    pool.spawn(async {
        abortable_render.await.ok();
    })
    .expect("GUI to be spawned");
    trigger
}

// This only actually happens if someone sends a signal using Kill - the GUI
// should handle input events to emulate Ctrl+C
fn abort_gui_on_signal(trigger: AbortHandle) {
    let signals = Signals::new(&[signal_hook::SIGINT, signal_hook::SIGTERM])
        .expect("signals to be set successfully");
    std::thread::spawn({
        let trigger = trigger.clone();
        move || {
            for signal in &signals {
                match signal {
                    signal_hook::SIGINT | signal_hook::SIGTERM => {
                        trigger.abort();
                        // DEBUG: after a crash, you really want to abort the program, it's stuck
                        std::process::abort();
                        return;
                    }
                    _ => unreachable!(),
                }
            }
        }
    });
}

fn main() -> Result {
    env_logger::init();
    // Use spawn as well to simulate Send futures
    let pool = futures::executor::ThreadPool::builder()
        .pool_size(1)
        .create()
        .expect("pool creation to work (io-error is not Send");
    futures::executor::block_on(work_forever(pool))
}

struct NestingLevel(u8);
type Result = std::result::Result<(), Box<dyn Error + Send>>;
