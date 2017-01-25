use clap;
use super::error::Error;
use structs::Crate;
use tokio_core::reactor;
use futures::{self, Future, BoxFuture, IntoFuture};
use std::sync::{Mutex, Arc};
use tokio_curl::Session;
use prettytable::{format, Table};

// fetch user info, user->user-id, then
// curl 'https://crates.io/api/v1/crates?page=1&per_page=10&user_id=980&sort=asc' -H 'Accept:
// application/json'
pub fn by_user(args: &clap::ArgMatches,
               session: Arc<Mutex<Session>>)
               -> BoxFuture<Vec<Crate>, Error> {
    futures::finished(vec![Crate {
                               description: Some("description".to_owned()),
                               downloads: 123,
                               max_version: "1.1".to_owned(),
                               name: "name".to_owned(),
                           }])
        .boxed()
}

pub fn handle_list<F, R>(_args: &clap::ArgMatches,
                         scmd_args: &clap::ArgMatches,
                         make_future: F)
                         -> Result<(), Error>
    where F: FnOnce(&clap::ArgMatches, Arc<Mutex<Session>>) -> R,
          R: IntoFuture<Item = Vec<Crate>, Error = Error>
{
    let mut reactor = reactor::Core::new().map_err(Error::ReactorInit)?;
    let session = Arc::new(Mutex::new(Session::new(reactor.handle())));
    let fut =
        make_future(scmd_args, session.clone()).into_future().and_then(|crates: Vec<Crate>| {
            if !crates.is_empty() {
                let table = {
                    let mut t = Table::new();
                    t.set_titles(row![b -> "Name", b -> "Description", b -> "Downloads",
                b -> "MaxVersion"]);
                    t.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
                    crates.into_iter().fold(t, |mut t, c| {
                        t.add_row(row![c.name,
                                       c.description
                                           .unwrap_or_else(|| {
                                               String::from("no description provided")
                                           }),
                                       c.downloads,
                                       c.max_version]);
                        t
                    })
                };
                table.print_tty(false);
            }
            Ok(())
        });
    reactor.run(fut)
}
