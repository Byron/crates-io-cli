#[macro_use]
extern crate quick_error;

pub mod error;

use crate::error::Error;
use crates_index_diff::Index;
use std::{path::Path, time::Instant};

pub type Result<T> = std::result::Result<T, Error>;

/// Runs the statistics and mining engine.
/// May run for a long time unless a deadline is specified.
/// Even though timeouts can be achieved from outside of the future, knowing the deadline may be used
/// by the engine to manage its time even more efficiently.
pub async fn run(
    db: impl AsRef<Path>,
    crates_io_path: impl AsRef<Path>,
    _deadline: Option<Instant>,
) -> Result<()> {
    let _index = Index::from_path_or_cloned(crates_io_path)?;
    let _db = sled::open(db)?;
    unimplemented!()
}

#[cfg(feature = "with-executor")]
/// For convenience, run the engine and block until done.
pub fn run_blocking(
    db: impl AsRef<Path>,
    crates_io_path: impl AsRef<Path>,
    deadline: Option<Instant>,
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
