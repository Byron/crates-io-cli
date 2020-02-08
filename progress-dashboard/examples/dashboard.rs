use futures::task::{Spawn, SpawnExt};
use futures_timer::Delay;
use progress_dashboard::TreeRoot;
use rand::random;
use std::error::Error;

async fn work_item(mut progress: TreeRoot) -> () {
    let max: u8 = random();
    progress.init(if max > 100 {
        None
    } else {
        Some((max % 100).into())
    });
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
    for id in 0..max_level as usize * 10 {
        pool.spawn(work_item(tree.add_child(format!("work {}", id + 1))))
            .expect("spawn to work");
    }

    let subtree = tree.add_child(format!("Level {}", current + 1));

    Ok(())
}

async fn do_work(pool: impl Spawn + Clone + Send + 'static) -> Result {
    let progress = progress_dashboard::Config::default().create();
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
