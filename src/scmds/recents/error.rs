use std::io;
use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Could not initialize tokio event loop in worker thread")]
    ThreadingError(#[from] io::Error),
    #[error(transparent)]
    EncodeError(#[from] serde_json::Error),
    #[error("Could not create directory to contain crates.io repository at '{}'", path.display())]
    RepositoryDirectory { source: io::Error, path: PathBuf },
    #[error(transparent)]
    IndexDiff(#[from] crates_index_diff::index::diff::Error),
    #[error(transparent)]
    IndexInit(#[from] crates_index_diff::index::init::Error),
}
