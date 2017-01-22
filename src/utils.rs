use futures::{Poll, Future};
use curl::easy::Easy;
use tokio_curl::{PerformError, Session};

use curl;
use futures;
use std::sync::Mutex;
use std::fmt::{self, Display};
use std::error::Error;
use std::default::Default;
use std::sync::Arc;
use std::sync::atomic::{Ordering, AtomicUsize};

use std;

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

pub fn remote_call<'a>(url: &str,
                       session: &Session)
                       -> futures::BoxFuture<(Arc<Mutex<Vec<u8>>>, Easy), RemoteCallError> {
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

    session.perform(req)
        .map(move |res| (buf, res))
        .map_err(move |e| {
            //               info(&format!("Request to {} failed with error: '{}'", url, e));
            e.into()
        })
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
    }
}
