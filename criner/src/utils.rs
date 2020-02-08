use crate::error::{Error, FormatDeadline, Result};
use futures::task::SpawnExt;
use futures::{
    future::{self, Either},
    task::Spawn,
};
use futures_timer::Delay;
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
    F: Future<Output = T> + Unpin,
{
    match deadline {
        Some(d) => {
            let selector = future::select(
                Delay::new(d.duration_since(SystemTime::now()).unwrap_or_default()),
                f,
            );
            match selector.await {
                Either::Left((_, _f)) => Err(Error::DeadlineExceeded(FormatDeadline(d))),
                Either::Right((r, _delay)) => Ok(r),
            }
        }
        None => Ok(f.await),
    }
}

pub async fn enforce_blocking<F, T>(deadline: Option<SystemTime>, f: F, s: impl Spawn) -> Result<T>
where
    F: FnOnce() -> T + Send + 'static,
    T: Send + 'static,
{
    enforce(deadline, s.spawn_with_handle(async { f() })?).await
}
