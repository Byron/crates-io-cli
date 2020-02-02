use super::error::Error;
use crate::{
    args::OutputKind,
    http_utils::{json_to_stdout, paged_crates_io_remote_call, CallMetaData, CallResult},
    structs::{Crate, Crates, Meta},
};
use clap;
use futures::{Future, IntoFuture};
use prettytable::{format, Table};
use std::sync::{Arc, Mutex};
use tokio_core::reactor;
use tokio_curl::Session;
use urlencoding;

fn crates_from_callresult_buf(buf: &[u8]) -> Result<(Vec<Crate>, Meta), Error> {
    let Crates { crates, meta } = serde_json::from_slice(buf)?;
    Ok((crates, meta))
}

fn crates_from_callresult(c: CallResult) -> Result<(Vec<Crate>, Meta), Error> {
    crates_from_callresult_buf(&c.0.lock().unwrap())
}

fn crates_merge(mut r: Vec<Crate>, c: CallResult) -> Result<Vec<Crate>, Error> {
    crates_from_callresult(c).map(|(mut res, _)| {
        r.append(&mut res);
        r
    })
}

fn crates_extract(c: CallResult) -> Result<(CallMetaData, Vec<Crate>), Error> {
    crates_from_callresult(c).map(|(crates, meta)| {
        (
            CallMetaData {
                total: meta.total,
                items: crates.len() as u32,
            },
            crates,
        )
    })
}

pub fn by_user(
    id: u32,
    session: Arc<Mutex<Session>>,
) -> Box<dyn Future<Item = Vec<Crate>, Error = Error> + Send> {
    Box::new(
        paged_crates_io_remote_call(
            &format!(
                "https://crates.io/api/v1/crates?user_id={}",
                urlencoding::encode(&format!("{}", id))
            ),
            None,
            session.clone(),
            crates_merge,
            crates_extract,
        )
        .map_err(Into::into),
    )
}

pub fn handle_list<F, R>(output_format: OutputKind, do_work: F) -> Result<(), Error>
where
    F: FnOnce(Arc<Mutex<Session>>) -> R,
    R: IntoFuture<Item = Vec<Crate>, Error = Error>,
{
    let mut reactor = reactor::Core::new().map_err(Error::ReactorInit)?;
    let session = Arc::new(Mutex::new(Session::new(reactor.handle())));
    let fut = do_work(session.clone())
        .into_future()
        .and_then(|crates: Vec<Crate>| {
            match output_format {
                OutputKind::human => {
                    if crates.is_empty() {
                        return Ok(());
                    }
                    let (mut table, titles) = {
                        let mut t = Table::new();
                        t.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
                        let mut total = 0;
                        let t = crates.into_iter().fold(t, |mut t, c| {
                            total += c.downloads;
                            t.add_row(row![
                                c.name,
                                c.description.unwrap_or_default(),
                                c.downloads,
                                c.max_version
                            ]);
                            t
                        });
                        (
                            t,
                            row![b -> "Name", b -> "Description", b ->
                            format!("Downloads (total={})" , total), b -> "MaxVersion"],
                        )
                    };
                    table.set_titles(titles);
                    table.print_tty(false);
                }
                OutputKind::json => json_to_stdout(&crates),
            }
            Ok(())
        });
    reactor.run(fut)
}

#[test]
fn test_crates_from_callresult() {
    let buf = include_bytes!("../../../tests/fixtures/byrons-crates.json");
    let (crates, meta) = crates_from_callresult_buf(buf).unwrap();
    assert_eq!(meta.total, 244);
    assert_eq!(crates.len(), 10);
}
