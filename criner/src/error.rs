use crates_index_diff::git2;
use sled;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Git2(err: git2::Error) {
            from()
            cause(err)
        }
        Sled(err: sled::Error) {
            from()
            cause(err)
        }
    }
}
