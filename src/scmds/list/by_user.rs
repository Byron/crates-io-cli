use structs::Crate;
use super::error::Error;
use futures::future::BoxFuture;
use futures::{self, Future};
use tokio_core::reactor::Handle;

// fetch user info, user->user-id, then
// curl 'https://crates.io/api/v1/crates?page=1&per_page=10&user_id=980&sort=asc' -H 'Accept:
// application/json'
pub fn by_user(handle: Handle) -> BoxFuture<Vec<Crate>, Error> {
    futures::finished(Vec::new()).boxed()
}
