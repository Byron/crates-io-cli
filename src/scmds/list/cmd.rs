use clap;
use super::error::Error;
use structs::Crate;
use tokio_core::reactor;
use futures::IntoFuture;

pub fn handle_list<F, R>(args: &clap::ArgMatches, crates_future: F) -> Result<(), Error>
    where F: FnOnce(reactor::Handle) -> R,
          R: IntoFuture<Item = Vec<Crate>, Error = Error>
{
    let mut reactor = reactor::Core::new().map_err(Error::ReactorInit)?;

    Ok(())
}