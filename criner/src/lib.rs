#[macro_use]
extern crate quick_error;

pub mod error;

use crate::error::{DeadlineFormat, Error};
use crates_index_diff::{CrateVersion, Index};
use log::info;
use std::{path::Path, time::SystemTime};

pub type Result<T> = std::result::Result<T, Error>;

fn version_id(v: &CrateVersion) -> Vec<u8> {
    let mut id = Vec::with_capacity(v.name.len() + v.version.len() + 1);
    id.extend_from_slice(&v.name.as_bytes());
    id.push(b':');
    id.extend_from_slice(&v.version.as_bytes());
    id
}

fn check(deadline: Option<SystemTime>) -> Result<()> {
    deadline
        .map(|d| {
            if d >= SystemTime::now() {
                Err(Error::DeadlineExceeded(DeadlineFormat(d)))
            } else {
                Ok(())
            }
        })
        .unwrap_or(Ok(()))
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
    info!("Potentially cloning crates index - this can take a while…");
    let index = Index::from_path_or_cloned(crates_io_path)?;
    let db = sled::open(db)?;
    let meta = db.open_tree("crate_versions")?;

    info!("Fetching crates index to see changes");
    let crate_versions = index.fetch_changes()?;

    info!("Fetched {} changed crates", crate_versions.len());
    let check_interval = std::cmp::max(crate_versions.len() / 100, 1);
    // TODO: can this loop be expressed as stream to be awaited? It's so fast, it's barely needed
    for (versions_stored, version) in crate_versions.iter().enumerate() {
        meta.insert(version_id(&version), rmp_serde::to_vec(&version)?)?;
        if versions_stored % check_interval == 0 {
            info!(
                "Stored {} of {} crate versions in database",
                versions_stored + 1,
                crate_versions.len()
            );
            check(deadline)?;
        }
    }
    Ok(())
}

#[cfg(feature = "with-executor")]
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
