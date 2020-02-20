use crate::error::Result;
use crate::model;
use crate::model::Task;
use crate::persistence::{Db, TasksTree, TreeAccess};
use async_std::sync::Receiver;
use futures_timer::Delay;
use std::time::{Duration, SystemTime};

pub enum Scheduling {
    //   /// Considers work done if everything was done. Will block to assure that
    //    All,
    /// Considers the work done if at least one task was scheduled. Will block to wait otherwise.
    AtLeastOne,
    //    /// Prefer to never wait for workers to perform a task and instead return without having scheduled anything
    // NeverBlock,
}

pub enum AsyncResult {
    // /// The required scheduling cannot be fulfilled without blocking
    // WouldBlock,
    /// The minimal scheduling requirement was met
    Done,
}

pub async fn schedule_tasks(
    version: &model::CrateVersion<'_>,
    mut progress: prodash::tree::Item,
    _mode: Scheduling,
    download: &async_std::sync::Sender<DownloadTask>,
) -> Result<AsyncResult> {
    progress.init(Some(1), Some("task"));
    progress.set(1);
    progress.blocked(None);
    download
        .send(DownloadTask {
            name: version.name.to_owned().into(),
            semver: version.version.to_owned().into(),
        })
        .await;
    Ok(AsyncResult::Done)
}

pub struct DownloadTask {
    name: String,
    semver: String,
}

/// "https://crates.io/api/v1/crates/#{name}/#{version}/download"
pub async fn download(
    db: Db,
    mut progress: prodash::tree::Item,
    r: Receiver<DownloadTask>,
) -> Result<()> {
    progress.init(None, Some("Kb"));
    const TASK_NAME: &str = "download";
    const TASK_VERSION: &str = "1.0.0";
    let mut dummy = Task {
        stored_at: SystemTime::now(),
        process: TASK_NAME.into(),
        version: TASK_VERSION.into(),
        state: Default::default(),
    };

    let mut key = Vec::with_capacity(32);
    let tasks = db.tasks();

    while let Some(DownloadTask { name, semver, .. }) = r.recv().await {
        progress.set_name(format!("↓ {}:{}", name, semver));
        let mut kt = (name.as_str(), semver.as_str(), dummy);
        key.clear();

        TasksTree::key_to_buf(&kt, &mut key);
        dummy = kt.2;

        let mut task = tasks.update(&key, |_| ())?;
        task.process = "download".into();
        task.version = "1.0.0".into(); // careful - if this changes, we have to download everything again
        for it in 1..=10 {
            Delay::new(Duration::from_secs(1)).await;
            progress.set(it)
        }
        kt.2 = task;
        tasks.upsert(&kt)?;
    }
    Ok(())
}
