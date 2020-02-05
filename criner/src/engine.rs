use crate::{
    error::{Error, Result},
    persistence::{Db, TreeAccess},
    utils::*,
};
use async_std;
use crates_index_diff::Index;
use log::info;
use std::{
    path::Path,
    time::{Duration, SystemTime},
};

async fn process_changes(
    db: Db,
    crates_io_path: impl AsRef<Path>,
    deadline: Option<SystemTime>,
) -> Result<()> {
    let start = SystemTime::now();
    info!("Potentially cloning crates index - this can take a while…");
    let index = enforce_blocking(deadline, {
        let path = crates_io_path.as_ref().to_path_buf();
        || Index::from_path_or_cloned(path)
    })
    .await??;
    info!("Fetching crates index to see changes");
    let crate_versions = enforce_blocking(deadline, move || index.fetch_changes()).await??;

    info!("Fetched {} changed crates", crate_versions.len());
    let check_interval = std::cmp::max(crate_versions.len() / 100, 1);
    enforce_blocking(deadline, {
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
                if versions_stored % check_interval == 0 {
                    info!(
                        "Stored {} of {} crate versions in database",
                        versions_stored + 1,
                        crate_versions.len()
                    );
                }
            }
            context.update_today(|c| {
                c.durations.fetch_crate_versions += SystemTime::now()
                    .duration_since(start)
                    .unwrap_or_else(|_| Duration::default())
            })?;
            Ok::<_, Error>(())
        }
    })
    .await??;
    Ok(())
}

/// Runs the statistics and mining engine.
/// May run for a long time unless a deadline is specified.
/// Even though timeouts can be achieved from outside of the future, knowing the deadline may be used
/// by the engine to manage its time even more efficiently.
pub async fn run(
    db: impl AsRef<Path>,
    crates_io_path: impl AsRef<Path>,
    deadline: Option<SystemTime>,
) -> Result<()> {
    let start_of_computation = SystemTime::now();
    check(deadline)?;

    let db = Db::open(db)?;
    let res = {
        let db = db.clone();
        process_changes(db, crates_io_path, deadline).await
    };
    info!(
        "Wallclock elapsed: {}",
        humantime::format_duration(
            SystemTime::now()
                .duration_since(start_of_computation)
                .unwrap_or_default()
        )
    );
    //    info!("{:#?}", db.context()?.global()?);
    res
}

/// For convenience, run the engine and block until done.
pub fn run_blocking(
    db: impl AsRef<Path>,
    crates_io_path: impl AsRef<Path>,
    deadline: Option<SystemTime>,
) -> Result<()> {
    async_std::task::block_on(run(db, crates_io_path, deadline))
}
