#[macro_use]
extern crate quick_error;

pub mod error;

use crate::error::{DeadlineFormat, Error};
use async_std::{
    future,
    stream::{self, StreamExt},
    task,
};
use crates_index_diff::{CrateVersion, Index};
use log::info;
use std::{future::Future, path::Path, time::SystemTime};

pub type Result<T> = std::result::Result<T, Error>;

fn version_id(v: &CrateVersion) -> Vec<u8> {
    let mut id = Vec::with_capacity(v.name.len() + v.version.len() + 1);
    id.extend_from_slice(&v.name.as_bytes());
    id.push(b':');
    id.extend_from_slice(&v.version.as_bytes());
    id
}

fn _check(deadline: Option<SystemTime>) -> Result<()> {
    deadline
        .map(|d| {
            if SystemTime::now() >= d {
                Err(Error::DeadlineExceeded(DeadlineFormat(d)))
            } else {
                Ok(())
            }
        })
        .unwrap_or(Ok(()))
}

async fn enforce<F, T>(deadline: Option<SystemTime>, f: F) -> Result<T>
where
    F: Future<Output = T>,
{
    match deadline {
        Some(d) => future::timeout(d.duration_since(SystemTime::now()).unwrap_or_default(), f)
            .await
            .map_err(|_| Error::DeadlineExceeded(DeadlineFormat(d))),
        None => Ok(f.await),
    }
}

async fn enforce_blocking<F, T>(deadline: Option<SystemTime>, f: F) -> Result<T>
where
    F: FnOnce() -> T + Send + 'static,
    T: Send + 'static,
{
    enforce(deadline, task::spawn_blocking(f)).await
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
    let res = async {
        info!("Potentially cloning crates index - this can take a while…");
        let index = enforce_blocking(deadline, {
            let path = crates_io_path.as_ref().to_path_buf();
            || Index::from_path_or_cloned(path)
        })
        .await??;
        let db = sled::open(db)?;
        let meta = db.open_tree("crate_versions")?;

        info!("Fetching crates index to see changes");
        let crate_versions = enforce_blocking(deadline, move || index.fetch_changes()).await??;

        info!("Fetched {} changed crates", crate_versions.len());
        let check_interval = std::cmp::max(crate_versions.len() / 100, 1);
        enforce(deadline, async {
            let mut crates_stream = stream::from_iter(crate_versions.iter().enumerate());
            while let Some((versions_stored, version)) = crates_stream.next().await {
                meta.insert(version_id(&version), rmp_serde::to_vec(&version)?)?;
                if versions_stored % check_interval == 0 {
                    info!(
                        "Stored {} of {} crate versions in database",
                        versions_stored + 1,
                        crate_versions.len()
                    );
                }
            }
            Ok::<_, Error>(())
        })
        .await??;
        Ok(())
    }
    .await;
    info!(
        "Wallclock elapsed: {}",
        humantime::format_duration(
            SystemTime::now()
                .duration_since(start_of_computation)
                .unwrap_or_default()
        )
    );
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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
