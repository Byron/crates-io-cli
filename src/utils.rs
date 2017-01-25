use futures::{Stream, Poll, Future, IntoFuture};
use curl::easy::Easy;
use tokio_curl::{PerformError, Session};

use curl;
use futures;
use std::cmp;
use std::sync::Mutex;
use std::fmt::{self, Display};
use std::error::Error;
use std::default::Default;
use std::sync::Arc;
use std::sync::atomic::{Ordering, AtomicUsize};

use std;

const MAX_ITEMS_PER_PAGE: u32 = 100;

#[derive(RustcDecodable, Clone)]
pub struct Dimension {
    pub width: u16,
    pub height: u16,
}

impl Dimension {
    pub fn loose_heigth(mut self, h: u16) -> Dimension {
        self.height -= h;
        self
    }
}

impl Default for Dimension {
    fn default() -> Dimension {
        #[cfg(windows)]
        fn imp() -> Dimension {
            Dimension {
                width: 80,
                height: 20,
            }
        }

        #[cfg(unix)]
        fn imp() -> Dimension {
            use termion::terminal_size;
            let (mw, mh) = terminal_size().unwrap_or((80, 20));
            Dimension {
                width: mw,
                height: mh,
            }
        }

        imp()
    }
}

struct WithCauses<'a>(&'a Error);

impl<'a> Display for WithCauses<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "ERROR: {}", self.0));
        let mut cursor = self.0;
        while let Some(err) = cursor.cause() {
            try!(write!(f, "\ncaused by: \n{}", err));
            cursor = err;
        }
        try!(write!(f, "\n"));
        Ok(())
    }
}

pub fn ok_or_exit<T, E>(result: Result<T, E>) -> T
    where E: Error
{
    match result {
        Ok(v) => v,
        Err(err) => {
            println!("{}", WithCauses(&err));
            std::process::exit(2);
        }
    }
}

#[must_use = "futures do nothing unless polled"]
pub struct DropOutdated<A>
    where A: Future
{
    inner: Option<A>,
    version: usize,
    current_version: Arc<AtomicUsize>,
}

pub enum DroppedOrError<T> {
    Dropped,
    Err(T),
}

impl<A> Future for DropOutdated<A>
    where A: Future
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

impl<A> DropOutdated<A>
    where A: Future
{
    pub fn with_version(f: A, version: Arc<AtomicUsize>) -> DropOutdated<A> {
        DropOutdated {
            inner: Some(f),
            version: version.load(Ordering::Relaxed),
            current_version: version,
        }
    }
}

pub type CallResult = (Arc<Mutex<Vec<u8>>>, Easy);
pub type RemoteCallFuture = futures::BoxFuture<CallResult, RemoteCallError>;

pub fn remote_call<'a>(url: &str, session: Arc<Mutex<Session>>) -> RemoteCallFuture {
    let mut req = Easy::new();
    if let Err(e) = req.get(true) {
        return futures::failed(e.into()).boxed();
    }
    if let Err(e) = req.url(&url) {
        return futures::failed(e.into()).boxed();
    }
    let buf = Arc::new(Mutex::new(Vec::new()));
    let buf_handle = buf.clone();
    if let Err(e) = req.write_function(move |data| {
        buf_handle.lock().unwrap().extend_from_slice(data);
        Ok(data.len())
    }) {
        return futures::failed(e.into()).boxed();
    };

    session.lock()
        .unwrap()
        .perform(req)
        .map(move |res| (buf, res))
        .map_err(move |e| e.into())
        .boxed()
}

quick_error! {
    #[derive(Debug)]
    pub enum RemoteCallError {
        Easy(err: curl::Error) {
            description("Easy curl could not be configured")
            from()
            cause(err)
        }
        Curl(err: PerformError) {
            description("A curl request failed")
            from()
            cause(err)
        }
        Any(err: Box<Error + Send + 'static>) {
            description("An error occurred")
            from()
            cause(&**err)
        }
    }
}

#[derive(RustcDecodable, Default)]
pub struct CallMetaData {
    /// total amount of items as reported by crates.io in `meta.total`
    pub total: u32,
    /// amount of items seen in the current call
    pub items: u32,
}

pub fn paged_crates_io_remote_call<T, M, E, Err>(url: &str,
                                                 max_items: Option<u32>,
                                                 session: Arc<Mutex<Session>>,
                                                 merge: M,
                                                 extract: E)
                                                 -> futures::BoxFuture<T, RemoteCallError>
    where T: Default + Send + 'static,
          Err: Error + Send + 'static,
          M: Fn(T, CallResult) -> Result<T, Err> + Send + Sync + 'static,
          E: FnOnce(CallResult) -> Result<(CallMetaData, T), Err> + Send + Sync + 'static
{
    let max_items = max_items.unwrap_or(u32::max_value());
    let page_size = cmp::min(MAX_ITEMS_PER_PAGE, max_items);

    let url = url.to_owned();
    remote_call(&format!("{}&per_page={}", url, page_size), session.clone())
        .and_then(move |r| {
            extract(r)
                .map_err(|e| RemoteCallError::Any(Box::new(e)))
                .into_future()
                .and_then(move |(m, initial)| {
                    let mut f = Vec::new();
                    let num_chunks = cmp::min(m.total.saturating_sub(m.items),
                                              max_items.saturating_sub(m.items)) /
                                     page_size;
                    let remainder = if m.total % page_size > 0 { 1 } else { 0 };
                    for ci in 0..num_chunks + remainder {
                        f.push(remote_call(&format!("{}&page={}&per_page={}",
                                                    url,
                                                    2 + ci,
                                                    page_size),
                                           session.clone()));
                    }
                    futures::stream::futures_unordered(f.into_iter()).fold(initial, move |m, r| {
                        merge(m, r).map_err(|e| RemoteCallError::Any(Box::new(e)))
                    })
                })
        })
        .boxed()
}
