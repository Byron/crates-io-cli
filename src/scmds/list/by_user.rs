use structs::Crate;
use super::error::Error;
use futures::future::BoxFuture;
use futures::{self, Future};
use tokio_core::reactor::Handle;

pub fn by_user(handle: Handle) -> BoxFuture<Vec<Crate>, Error> {
    futures::finished(Vec::new()).boxed()
}
