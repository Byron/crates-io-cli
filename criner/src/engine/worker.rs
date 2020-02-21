use crate::error::{Error, Result};
use crate::model;
use crate::model::{Task, TaskResult, TaskState};
use crate::persistence::{Db, TaskResultTree, TasksTree, TreeAccess};
use async_std::sync::Receiver;
use std::path::PathBuf;
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
            kind: "crate",
            url: format!(
                "https://crates.io/api/v1/crates/{name}/{version}/download",
                name = version.name,
                version = version.version
            ),
        })
        .await;
    Ok(AsyncResult::Done)
}

pub struct DownloadTask {
    name: String,
    semver: String,
    kind: &'static str,
    url: String,
}

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
    let mut body_buf = Vec::new();

    while let Some(DownloadTask {
        name,
        semver,
        kind,
        url,
    }) = r.recv().await
    {
        progress.set_name(format!("↓ {}:{}", name, semver));
        progress.init(None, None);
        let mut kt = (name.as_str(), semver.as_str(), dummy);
        key.clear();

        TasksTree::key_to_buf(&kt, &mut key);
        dummy = kt.2;

        let mut task = tasks.update(&key, |_| ())?;
        task.process = TASK_NAME.into();
        task.version = TASK_VERSION.into();

        progress.blocked(None);
        let res = {
            let mut res = reqwest::get(&url).await?;
            let size: u32 =
                res.content_length()
                    .ok_or(Error::InvalidHeader("expected content-length"))? as u32;
            progress.init(Some(size / 1024), Some("Kb"));
            progress.blocked(None);
            progress.done(format!("HEAD:{}: content-size = {}", url, size));
            body_buf.clear();
            while let Some(chunk) = res.chunk().await? {
                body_buf.extend(chunk);
                progress.set((body_buf.len() / 1024) as u32);
            }
            progress.done(format!("GET:{}: body-size = {}", url, body_buf.len()));

            {
                key.clear();
                let insert_item = (
                    name.as_str(),
                    semver.as_str(),
                    &task,
                    TaskResult::Download {
                        kind: kind.into(),
                        url: url.as_str().into(),
                        content_length: size,
                        content_type: res
                            .headers()
                            .get(http::header::CONTENT_TYPE)
                            .and_then(|t| t.to_str().ok())
                            .map(Into::into),
                        data: Some(body_buf.as_slice().into()),
                    },
                );
                TaskResultTree::key_to_buf(&insert_item, &mut key);
                store_data(db.results(), &key, insert_item).await?;
            }
            Ok(())
        }
        .map_err(|e: crate::error::Error| {
            let e = e.to_string();
            progress.fail(format!("Failed to download '{}': {}", url, e));
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

async fn store_data(
    tree: TaskResultTree<'_>,
    key: &[u8],
    res: (&str, &str, &Task<'_>, TaskResult<'_>),
) -> Result<()> {
    tree.insert(&res)?;
    let key_str = String::from_utf8(key.to_owned())?;
    // For now, we store a backup and to make manual inspection easier…
    Ok(match res.3 {
        TaskResult::Download {
            data: Some(data), ..
        } => tokio::fs::write(PathBuf::from("./criner.db/assets").join(&key_str), data).await?,
        _ => (),
    })
}
