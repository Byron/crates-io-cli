use crates_index_diff::git2;
quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Git2(err: git2::Error) {
            description("A git operation failed")
            from()
            cause(err)
        }
    }
}
