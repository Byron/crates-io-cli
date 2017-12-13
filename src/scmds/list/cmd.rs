use clap;
use super::error::Error;
use structs::{Crate, Meta, OutputKind};
use tokio_core::reactor;
use futures::{Future, IntoFuture};
use std::sync::{Arc, Mutex};
use tokio_curl::Session;
use prettytable::{format, Table};
use utils::{json_to_stdout, paged_crates_io_remote_call, CallMetaData, CallResult};
use urlencoding;
use rustc_serialize::json::{Decoder, DecoderError, Json};
use rustc_serialize::Decodable;
use std::str;

const TOTAL_PATH: [&'static str; 2] = ["meta", "total"];
const CRATES_PATH: [&'static str; 1] = ["crates"];

fn crates_from_callresult_buf(buf: &[u8]) -> Result<(Vec<Crate>, Meta), Error> {
    fn at_path<'a>(json: &'a Json, p: &[&str]) -> Result<&'a Json, DecoderError> {
        json.find_path(&p)
            .ok_or_else(|| DecoderError::ApplicationError(format!("Missing path: {:?}", p)))
    }
    str::from_utf8(buf)
        .map_err(|e| Error::Decode(DecoderError::ApplicationError(format!("{}", e))))
        .and_then(|s| {
            Json::from_str(s)
                .map_err(DecoderError::ParseError)
                .map_err(Into::into)
        })
        .and_then(|json| {
            let total = at_path(&json, &TOTAL_PATH).and_then(|json| {
                json.as_u64().map(|v| v as u32).ok_or_else(|| {
                    DecoderError::ApplicationError(format!("Expected integer, found {}", json))
                })
            })?;
            at_path(&json, &CRATES_PATH)?;
            let crates = json.into_object()
                .expect("top level object")
                .remove("crates")
                .expect("crates entry");
            let mut decoder = Decoder::new(crates);
            Ok((Decodable::decode(&mut decoder)?, Meta { total: total }))
        })
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
    args: &clap::ArgMatches,
    session: Arc<Mutex<Session>>,
) -> Box<Future<Item = Vec<Crate>, Error = Error> + Send> {
    let uid = args.value_of("user-id").expect("clap to work");
    Box::new(
        paged_crates_io_remote_call(
            &format!(
                "https://crates.io/api/v1/crates?user_id={}",
                urlencoding::encode(uid)
            ),
            None,
            session.clone(),
            crates_merge,
            crates_extract,
        ).map_err(Into::into),
    )
}

pub fn handle_list<F, R>(
    args: &clap::ArgMatches,
    scmd_args: &clap::ArgMatches,
    make_future: F,
) -> Result<(), Error>
where
    F: FnOnce(&clap::ArgMatches, Arc<Mutex<Session>>) -> R,
    R: IntoFuture<Item = Vec<Crate>, Error = Error>,
{
    let mut reactor = reactor::Core::new().map_err(Error::ReactorInit)?;
    let session = Arc::new(Mutex::new(Session::new(reactor.handle())));
    let output_kind: OutputKind = args.value_of("format")
        .expect("default to be set")
        .parse()
        .expect("clap to work");
    let fut = make_future(scmd_args, session.clone())
        .into_future()
        .and_then(|crates: Vec<Crate>| {
            match output_kind {
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
                                c.description
                                    .unwrap_or_else(|| String::from("no description provided")),
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
