use crate::error::Result;
use crate::model;
use crate::model::{Task, TaskState};
use crate::persistence::{Db, TasksTree, TreeAccess};
use async_std::sync::Receiver;
use std::time::SystemTime;

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
        task.process = TASK_NAME.into();
        task.version = TASK_VERSION.into();

        progress.blocked(None);
        let download_url = format!(
            "https://crates.io/api/v1/crates/{name}/{version}/download",
            name = name,
            version = semver
        );
        let res = {
            // let size: u32 = response
            //     .header("Content-Length")
            //     .ok_or(Error::InvalidHeader("expected content-length"))
            //     .and_then(|l| l.parse().map_err(Into::into))?;
            Ok(())
        }
        .map_err(|e: crate::error::Error| {
            let e = e.to_string();
            progress.fail(format!("Failed to download '{}': {}", download_url, e));
            e
        });

        task.state = match res {
            Ok(_) => TaskState::Complete,
            Err(err) => TaskState::AttemptsWithFailure(vec![err]),
        };
        kt.2 = task;
        tasks.upsert(&kt)?;
    }
    Ok(())
}
