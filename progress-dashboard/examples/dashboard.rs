use futures::future::AbortHandle;
use futures::task::{Spawn, SpawnExt};
use futures_timer::Delay;
use log::info;
use progress_dashboard::{tui, TreeRoot};
use rand::prelude::*;
use signal_hook::iterator::Signals;
use std::{error::Error, time::Duration};

const MAX_STEPS: u8 = 100;
const UNITS: &[&str] = &["Mb", "kb", "items", "files"];
const WORK_DELAY_MS: u64 = 100;
const SPAWN_DELAY_MS: u64 = 500;

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
        for id in 0..max_level as usize * 2 {
            pool.spawn(work_item(tree.add_child(format!("work {}", id + 1))))
                .expect("spawn to work");
            info!("spawn work: wait");
            Delay::new(Duration::from_millis(SPAWN_DELAY_MS)).await;
        }
        tree = tree.add_child(format!("Level {}", level + 1));
    }

    Ok(())
}

async fn do_work(pool: impl Spawn + Clone + Send + 'static) -> Result {
    let progress = progress_dashboard::TreeRoot::new();
    let local_work = find_work(NestingLevel(2), progress.clone(), pool.clone());
    let threaded_work = pool
        .spawn_with_handle(find_work(NestingLevel(2), progress.clone(), pool.clone()))
        .expect("spawning to work - SpawnError cannot be ");

    // Now we should handle signals to be able to cleanup properly
    let (abortable_render, trigger) = futures::future::abortable(tui::render(
        progress,
        tui::Config {
            frames_per_second: 30,
        },
    ));

    pool.spawn(async {
        abortable_render.await.ok();
    })
    .expect("GUI to be spawned");

    abort_gui_on_signal(trigger.clone());
    let res = futures::future::join(local_work, threaded_work).await.0;
    trigger.abort();
    res
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
                        trigger.abort()
                    }
                    _ => unreachable!(),
                }
            }
        }
    });
}

fn main() -> Result {
    // Use spawn as well to simulate Send futures
    let pool = futures::executor::ThreadPool::builder()
        .pool_size(1)
        .create()
        .expect("pool creation to work (io-error is not Send");
    futures::executor::block_on(do_work(pool))
}

struct NestingLevel(u8);
type Result = std::result::Result<(), Box<dyn Error + Send>>;
