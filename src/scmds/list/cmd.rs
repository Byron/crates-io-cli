use super::error::Error;
use crate::{
    args::OutputKind,
    http_utils::{CallMetaData, CallResult, paged_crates_io_remote_call},
    structs::{Crate, Crates, Meta},
};
use prettytable::{Table, format};
use std::io;
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

pub fn by_user(id: u32) -> Result<Vec<Crate>, Error> {
    paged_crates_io_remote_call(
        &format!(
            "https://crates.io/api/v1/crates?user_id={}",
            urlencoding::encode(&format!("{}", id))
        ),
        None,
        crates_merge,
        crates_extract,
        None,
    )
    .map_err(Into::into)
}

pub fn handle_list<F, R>(output_format: OutputKind, do_work: F) -> Result<(), Error>
where
    F: FnOnce() -> R,
    R: Into<Result<Vec<Crate>, Error>>,
{
    let crates = do_work().into()?;
    match output_format {
        OutputKind::Human => {
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
            table.print_tty(false)?;
            Ok(())
        }
        OutputKind::Json => serde_json::to_writer_pretty(io::stdout(), &crates).map_err(Into::into),
    }
}

#[test]
fn test_crates_from_callresult() {
    let buf = include_bytes!("../../../tests/fixtures/byrons-crates.json");
    let (crates, meta) = crates_from_callresult_buf(buf).unwrap();
    assert_eq!(meta.total, 244);
    assert_eq!(crates.len(), 10);
}
