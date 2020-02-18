use crate::error::Result;

pub enum Scheduling {
    //   /// Considers work done if everything was done. Will block to assure that
    //    All,
    //    /// Considers the work done if at least one task was scheduled. Will block to wait otherwise.
    //    AtLeastOne,
    /// Prefer to never wait for workers to perform a task and instead return without having scheduled anything
    NeverBlock,
}

pub enum AsyncResult {
    /// The required scheduling cannot be fulfilled without blocking
    WouldBlock,
    /// The minimal scheduling requirement was met
    Done,
}

pub async fn schedule_tasks(
    _version: &crates_index_diff::CrateVersion,
    mut progress: prodash::tree::Item,
    _mode: Scheduling,
) -> Result<AsyncResult> {
    progress.init(None, Some("tasks"));
    Ok(AsyncResult::WouldBlock)
}
