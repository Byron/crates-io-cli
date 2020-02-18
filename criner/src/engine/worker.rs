use crate::error::Result;
use async_std::sync::Receiver;
use futures_timer::Delay;
use std::time::Duration;

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
    download: &async_std::sync::Sender<()>,
) -> Result<AsyncResult> {
    progress.init(None, Some("tasks"));
    Ok(if download.is_full() {
        AsyncResult::WouldBlock
    } else {
        download.send(()).await;
        AsyncResult::Done
    })
}

pub async fn download(mut progress: prodash::tree::Item, r: Receiver<()>) -> () {
    progress.init(None, Some("Kb"));
    while let Some(()) = r.recv().await {
        for it in 1..=10 {
            Delay::new(Duration::from_secs(1)).await;
            progress.set(it)
        }
    }
}
