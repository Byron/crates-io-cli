use crate::error::Result;
use crate::model;
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
    version: &model::CrateVersion,
    mut progress: prodash::tree::Item,
    _mode: Scheduling,
    download: &async_std::sync::Sender<DownloadTask>,
) -> Result<AsyncResult> {
    progress.init(None, Some("tasks"));
    Ok(if download.is_full() {
        AsyncResult::WouldBlock
    } else {
        download
            .send(DownloadTask {
                name: format!("↓ {}:{}", version.name, version.version),
            })
            .await;
        AsyncResult::Done
    })
}

pub struct DownloadTask {
    name: String,
}

/// "https://crates.io/api/v1/crates/#{name}/#{version}/download"
pub async fn download(mut progress: prodash::tree::Item, r: Receiver<DownloadTask>) -> () {
    progress.init(None, Some("Kb"));
    while let Some(DownloadTask { name }) = r.recv().await {
        progress.set_name(name);
        for it in 1..=10 {
            Delay::new(Duration::from_secs(1)).await;
            progress.set(it)
        }
    }
}
