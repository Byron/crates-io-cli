#[macro_use]
extern crate quick_error;

pub mod error;

use crate::error::Error;
use crates_index_diff::Index;
use std::{path::Path, time::Duration};

pub type Result<T> = std::result::Result<T, Error>;

pub async fn run(
    _db: impl AsRef<Path>,
    crates_io_path: impl AsRef<Path>,
    _time_limit: Option<Duration>,
) -> Result<()> {
    let _index = Index::from_path_or_cloned(crates_io_path)?;
    unimplemented!()
}

pub fn run_blocking(
    db: impl AsRef<Path>,
    crates_io_path: impl AsRef<Path>,
    time_limit: Option<Duration>,
) -> Result<()> {
    async_std::task::block_on(run(db, crates_io_path, time_limit))
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
