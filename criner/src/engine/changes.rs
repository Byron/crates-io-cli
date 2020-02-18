use crate::{
    engine::worker::{schedule_tasks, AsyncResult, Scheduling},
    error::{Error, Result},
    persistence::CrateVersionsTree,
    persistence::{Db, TreeAccess},
    utils::*,
};
use crates_index_diff::Index;
use futures::task::Spawn;
use std::{
    path::Path,
    time::{Duration, SystemTime},
};

pub async fn process(
    db: Db,
    crates_io_path: impl AsRef<Path>,
    deadline: Option<SystemTime>,
    pool: impl Spawn,
    mut progress: prodash::tree::Item,
) -> Result<()> {
    let start = SystemTime::now();
    let mut subprogress =
        progress.add_child("Potentially cloning crates index - this can take a while…");
    let index = enforce_blocking(
        deadline,
        {
            let path = crates_io_path.as_ref().to_path_buf();
            || Index::from_path_or_cloned(path)
        },
        &pool,
    )
    .await??;
    subprogress.set_name("Fetching crates index to see changes");
    let crate_versions = enforce_blocking(deadline, move || index.fetch_changes(), &pool).await??;

    progress.done(format!("Fetched {} changed crates", crate_versions.len()));
    drop(subprogress);

    let mut store_progress = progress.add_child("processing new crates");
    store_progress.init(Some(crate_versions.len() as u32), Some("crate versions"));

    enforce_future(
        deadline,
        {
            let db = db.clone();
            async move {
                let versions = db.open_crate_versions()?;
                let krate = db.open_crates()?;
                let context = db.context();
                let mut may_schedule_tasks = true;
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

                    // There is enough scheduling capacity for this not to block
                    // TODO: one day we may decide based on other context whether to continue
                    // blocking while trying, or not, or try again a bit later after storing
                    // a chunk of versions
                    if may_schedule_tasks {
                        let res = schedule_tasks(
                            version,
                            store_progress.add_child(format!("schedule {}", CrateVersionsTree::key_str(version))),
                            Scheduling::NeverBlock,
                        )
                            .await?;
                        if let AsyncResult::WouldBlock = res {
                            store_progress.info("Skipping further task scheduling in preference for storing new versions");
                            may_schedule_tasks = false;
                        }
                    }
                    store_progress.set((versions_stored + 1) as u32);
                }
                context.update_today(|c| {
                    c.durations.fetch_crate_versions += SystemTime::now()
                        .duration_since(start)
                        .unwrap_or_else(|_| Duration::default())
                })?;
                store_progress.done(format!(
                    "Stored {} crate versions to database",
                    crate_versions.len()
                ));
                Ok::<_, Error>(())
            }
        },
        &pool,
    )
        .await??;
    Ok(())
}
