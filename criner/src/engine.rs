use crate::{
    error::{Error, Result},
    persistence::{Db, TreeAccess},
    utils::*,
};
use crates_index_diff::Index;
use futures::executor::LocalPool;
use futures::task::SpawnExt;
use futures::{executor::ThreadPool, future::FutureExt, task::Spawn};
use log::info;
use std::path::PathBuf;
use std::{
    path::Path,
    time::{Duration, SystemTime},
};

async fn process_changes(
    db: Db,
    crates_io_path: impl AsRef<Path>,
    deadline: Option<SystemTime>,
    pool: impl Spawn,
    mut progress: prodash::tree::Item,
) -> Result<()> {
    let start = SystemTime::now();
    progress.info("Potentially cloning crates index - this can take a while…");
    progress.init(None, None);
    progress.blocked(None);
    let index = enforce_blocking(
        deadline,
        {
            let path = crates_io_path.as_ref().to_path_buf();
            || Index::from_path_or_cloned(path)
        },
        &pool,
    )
    .await??;
    progress.info("Fetching crates index to see changes");
    let crate_versions = enforce_blocking(deadline, move || index.fetch_changes(), &pool).await??;

    progress.done(format!("Fetched {} changed crates", crate_versions.len()));
    let mut store_progress = progress.add_child("processing new crates");
    store_progress.init(Some(crate_versions.len() as u32), Some("crates"));

    enforce_blocking(
        deadline,
        {
            let db = db.clone();
            move || {
                let versions = db.open_crate_versions()?;
                let krate = db.open_crates()?;
                let context = db.context()?;
                // NOTE: this loop can also be a stream, but that makes computation slower due to overhead
                // Thus we just do this 'quickly' on the main thread, knowing that criner really needs its
                // own executor or resources.
                // We could chunk things, but that would only make the code harder to read. No gains here…
                // NOTE: Even chunks of 1000 were not faster, didn't even saturate a single core...
                for (versions_stored, version) in crate_versions.iter().enumerate() {
                    // NOTE: For now, not transactional, but we *could*!
                    {
                        versions.insert(&version)?;
                        context.update_today(|c| c.counts.crate_versions += 1)?;
                    }
                    if krate.upsert(&version)? {
                        context.update_today(|c| c.counts.crates += 1)?;
                    }
                    store_progress.set((versions_stored + 1) as u32);
                }
                context.update_today(|c| {
                    c.durations.fetch_crate_versions += SystemTime::now()
                        .duration_since(start)
                        .unwrap_or_else(|_| Duration::default())
                })?;
                Ok::<_, Error>(())
            }
        },
        &pool,
    )
    .await??;
    Ok(())
}

/// Runs the statistics and mining engine.
/// May run for a long time unless a deadline is specified.
/// Even though timeouts can be achieved from outside of the future, knowing the deadline may be used
/// by the engine to manage its time even more efficiently.
pub async fn run(
    db: Db,
    crates_io_path: PathBuf,
    deadline: Option<SystemTime>,
    progress: prodash::Tree,
    pool: impl Spawn,
) -> Result<()> {
    let start_of_computation = SystemTime::now();
    check(deadline)?;

    let res = {
        let db = db.clone();
        process_changes(
            db,
            crates_io_path,
            deadline,
            pool,
            progress.add_child("crates.io refresh"),
        )
        .await
    };

    info!(
        "Wallclock elapsed: {}",
        humantime::format_duration(
            SystemTime::now()
                .duration_since(start_of_computation)
                .unwrap_or_default()
        )
    );
    info!("{:#?}", db.context()?.iter().next_back().expect("one")?);
    res
}

/// For convenience, run the engine and block until done.
pub fn run_blocking(
    db: impl AsRef<Path>,
    crates_io_path: impl AsRef<Path>,
    deadline: Option<SystemTime>,
) -> Result<()> {
    // NOTE: pool should be big enough to hold all possible blocking tasks running in parallel.
    // The main thread is expected to pool non-blocking tasks.
    // Of course, non-blocking tasks may also be scheduled there, which is when you probably want
    // to have another free thread just for that.
    // All this is theory.
    let pool_size = 2;
    let blocking_task_pool = ThreadPool::builder().pool_size(pool_size).create()?;
    let mut local_pool = LocalPool::new();

    let root = prodash::Tree::new();
    let gui = prodash::tui::render(
        root.clone(),
        prodash::tui::TuiOptions {
            title: "minimal example".into(),
            ..prodash::tui::TuiOptions::default()
        },
    )?;
    let db = Db::open(db)?;

    local_pool.spawner().spawn(
        run(
            db,
            crates_io_path.as_ref().into(),
            deadline,
            root,
            blocking_task_pool,
        )
        .map(|_| ()),
    )?;

    local_pool.run_until(gui);
    // at this point, we forget all currently running computation, and since it's in the local thread, it's all
    // destroyed/dropped properly.
    Ok(())
}
