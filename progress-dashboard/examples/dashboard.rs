use futures::task::{Spawn, SpawnExt};
use futures_timer::Delay;
use progress_dashboard::TreeRoot;
use rand::prelude::*;
use std::error::Error;
use std::time::Duration;

const MAX_STEPS: u8 = 100;
const UNITS: &[&str] = &["Mb", "kb", "items", "files"];

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
        Delay::new(Duration::from_millis(100)).await;
    }
    ()
}

async fn find_work(
    current: NestingLevel,
    max: NestingLevel,
    mut tree: TreeRoot,
    pool: impl Spawn,
) -> Result {
    let NestingLevel(current) = current;
    let NestingLevel(max_level) = max;
    if current > max_level {
        return Ok(());
    }

    // one-off ambient tasks
    for id in 0..max_level as usize * 2 {
        pool.spawn(work_item(tree.add_child(format!("work {}", id + 1))))
            .expect("spawn to work");
    }

    let subtree = tree.add_child(format!("Level {}", current + 1));
    find_work(NestingLevel(current + 1), max, subtree, pool).await
}

async fn do_work(pool: impl Spawn + Clone + Send + 'static) -> Result {
    let progress = progress_dashboard::TreeRoot::new();
    let local_work = find_work(
        NestingLevel(0),
        NestingLevel(2),
        progress.clone(),
        pool.clone(),
    );
    let threaded_work = pool
        .spawn_with_handle(find_work(
            NestingLevel(0),
            NestingLevel(2),
            progress,
            pool.clone(),
        ))
        .expect("spawning to work - SpawnError cannot be ");
    futures::future::join(local_work, threaded_work).await.0
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
