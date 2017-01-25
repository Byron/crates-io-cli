use clap;
use super::error::Error;
use structs::{Meta, Crate};
use tokio_core::reactor;
use futures::{self, Future, BoxFuture, IntoFuture};
use std::sync::{Mutex, Arc};
use tokio_curl::Session;
use prettytable::{format, Table};
use utils::{CallMetaData, CallResult, paged_crates_io_remote_call};
use urlencoding;

fn crates_from_callresult(c: CallResult) -> Result<(Vec<Crate>, Meta), Error> {
    Ok((Vec::new(), Meta { total: 0 }))
}

fn crates_merge(mut r: Vec<Crate>, c: CallResult) -> Result<Vec<Crate>, Error> {
    crates_from_callresult(c).map(|(mut res, _)| {
        r.append(&mut res);
        r
    })
}

fn crates_extract(c: CallResult) -> Result<(CallMetaData, Vec<Crate>), Error> {
    crates_from_callresult(c).map(|(crates, meta)| {
        (CallMetaData {
             total: meta.total,
             items: crates.len() as u32,
         },
         crates)
    })
}

pub fn by_user(args: &clap::ArgMatches,
               session: Arc<Mutex<Session>>)
               -> BoxFuture<Vec<Crate>, Error> {
    let uid = args.value_of("user-id").expect("clap to work");
    paged_crates_io_remote_call(&format!("https://crates.io/api/v1/crates?user_id={}",
                                         urlencoding::encode(uid)),
                                None,
                                session.clone(),
                                crates_merge,
                                crates_extract)
        .map_err(Into::into)
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
