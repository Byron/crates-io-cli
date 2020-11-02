use crates_index_diff::git2;
use std::io;
use std::path::PathBuf;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        ThreadingError(err: io::Error) {
            description("Could not initialize tokio event loop in worker thread")
            cause(err)
        }
        EncodeError(err: serde_json::Error) {
            cause(err)
            from()
        }
        RepositoryDirectory(err: io::Error, path: PathBuf) {
            display("Could not create directory to contain crates.io repository at '{}'",
                     path.display())
            cause(err)
        }
        Git2(err: git2::Error) {
            description("A git operation failed")
            from()
            cause(err)
        }
    }
}
