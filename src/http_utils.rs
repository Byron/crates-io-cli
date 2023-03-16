use curl::easy::Easy;
use futures::{Future, IntoFuture, Poll, Stream};
use tokio_curl::{PerformError, Session};

use curl;
use futures;
use std::{
    cmp,
    default::Default,
    error::Error,
    sync::atomic::{AtomicUsize, Ordering},
    sync::Arc,
    sync::Mutex,
};

const MAX_ITEMS_PER_PAGE: u32 = 100;

#[must_use = "futures do nothing unless polled"]
pub struct DropOutdated<A>
where
    A: Future,
{
    pub(crate) inner: Option<A>,
    pub(crate) version: usize,
    pub(crate) current_version: Arc<AtomicUsize>,
}

pub enum DroppedOrError<T> {
    Dropped,
    Err(T),
}

impl<A> Future for DropOutdated<A>
where
    A: Future,
{
    type Item = A::Item;
    type Error = DroppedOrError<A::Error>;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        let v = self.current_version.load(Ordering::Relaxed);
        if v != self.version {
            drop(self.inner.take());
        }
        match self.inner {
            Some(ref mut f) => f.poll().map_err(|e| DroppedOrError::Err(e)),
            None => Err(DroppedOrError::Dropped),
        }
    }
}

pub type CallResult = (Arc<Mutex<Vec<u8>>>, Easy);
pub type RemoteCallFuture =
    Box<dyn futures::Future<Item = CallResult, Error = RemoteCallError> + Send>;

pub fn remote_call<'a>(url: &str, session: Arc<Mutex<Session>>) -> RemoteCallFuture {
    let mut req = request_new();
    if let Err(e) = req.get(true) {
        return Box::new(futures::failed(e.into()));
    }
    if let Err(e) = req.url(&url) {
        return Box::new(futures::failed(e.into()));
    }
    let buf = Arc::new(Mutex::new(Vec::new()));
    let buf_handle = buf.clone();
    if let Err(e) = req.write_function(move |data| {
        buf_handle.lock().unwrap().extend_from_slice(data);
        Ok(data.len())
    }) {
        return Box::new(futures::failed(e.into()));
    };

    Box::new(
        session
            .lock()
            .unwrap()
            .perform(req)
            .map(move |res| (buf, res))
            .map_err(move |e| e.into()),
    )
}

fn request_new() -> Easy {
    let mut easy = Easy::new();
    easy.useragent("crates.io-cli (https://crates.io/crates/crates-io-cli)")
        .ok();
    easy
}

#[derive(Debug, thiserror::Error)]
pub enum RemoteCallError {
    #[error("Easy curl could not be configured")]
    Easy(#[from] curl::Error),
    #[error("A curl request failed")]
    Curl(#[from] PerformError),
    #[error("An error occurred")]
    Any(#[from] Box<dyn Error + Send + 'static>),
}

#[derive(Default)]
pub struct CallMetaData {
    /// total amount of items as reported by crates.io in `meta.total`
    pub total: u32,
    /// amount of items seen in the current call
    pub items: u32,
}

pub fn paged_crates_io_remote_call<T, M, E, Err>(
    url: &str,
    max_items: Option<u32>,
    session: Arc<Mutex<Session>>,
    merge: M,
    extract: E,
) -> Box<dyn futures::Future<Item = T, Error = RemoteCallError> + Send>
where
    T: Default + Send + 'static,
    Err: Error + Send + 'static,
    M: Fn(T, CallResult) -> Result<T, Err> + Send + Sync + 'static,
    E: FnOnce(CallResult) -> Result<(CallMetaData, T), Err> + Send + Sync + 'static,
{
    let max_items = max_items.unwrap_or(u32::max_value());
    let page_size = cmp::min(MAX_ITEMS_PER_PAGE, max_items);

    let url = url.to_owned();
    Box::new(
        remote_call(&format!("{}&per_page={}", url, page_size), session.clone()).and_then(
            move |r| {
                extract(r)
                    .map_err(|e| RemoteCallError::Any(Box::new(e)))
                    .into_future()
                    .and_then(move |(m, initial)| {
                        let mut f = Vec::new();
                        let num_chunks = cmp::min(
                            m.total.saturating_sub(m.items),
                            max_items.saturating_sub(m.items),
                        ) / page_size;
                        let remainder = if m.total % page_size > 0 { 1 } else { 0 };
                        for ci in 0..num_chunks + remainder {
                            f.push(remote_call(
                                &format!("{}&page={}&per_page={}", url, 2 + ci, page_size),
                                session.clone(),
                            ));
                        }
                        futures::stream::futures_unordered(f.into_iter())
                            .fold(initial, move |m, r| {
                                merge(m, r).map_err(|e| RemoteCallError::Any(Box::new(e)))
                            })
                    })
            },
        ),
    )
}
