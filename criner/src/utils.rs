use crate::error::{Error, FormatDeadline, Result};
use async_std::{future, task};
use std::{future::Future, time::SystemTime};

pub fn check(deadline: Option<SystemTime>) -> Result<()> {
    deadline
        .map(|d| {
            if SystemTime::now() >= d {
                Err(Error::DeadlineExceeded(FormatDeadline(d)))
            } else {
                Ok(())
            }
        })
        .unwrap_or(Ok(()))
}

pub async fn enforce<F, T>(deadline: Option<SystemTime>, f: F) -> Result<T>
where
    F: Future<Output = T>,
{
    match deadline {
        Some(d) => future::timeout(d.duration_since(SystemTime::now()).unwrap_or_default(), f)
            .await
            .map_err(|_| Error::DeadlineExceeded(FormatDeadline(d))),
        None => Ok(f.await),
    }
}

pub async fn enforce_blocking<F, T>(deadline: Option<SystemTime>, f: F) -> Result<T>
where
    F: FnOnce() -> T + Send + 'static,
    T: Send + 'static,
{
    enforce(deadline, task::spawn_blocking(f)).await
}
