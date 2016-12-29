extern crate curl;
extern crate termion;
extern crate futures;
extern crate tokio_core;
extern crate tokio_curl;
extern crate futures_cpupool;
extern crate url;

use clap;
use std::str;
use std::sync::{Mutex, Arc};
use self::url::percent_encoding::{DEFAULT_ENCODE_SET, percent_encode};
use rustc_serialize::json;
use std::thread;
use self::futures_cpupool::CpuPool;
use self::curl::easy::Easy;
use self::termion::event::Key;
use self::termion::raw::IntoRawMode;
use self::termion::input::TermRead;
use self::termion::clear;
use self::termion::cursor;
use self::tokio_core::reactor::Core;
use self::futures::{Sink, Stream, Future};
use self::futures::sync::mpsc;
use self::tokio_curl::Session;
use std::io::{self, Write};

use utils::ok_or_exit;

#[derive(RustcDecodable)]
struct Meta {
    total: u32,
}

#[derive(RustcDecodable)]
struct Crate {
    created_at: String,
    description: String,
    downloads: u32,
    max_version: String,
    name: String,
}

#[derive(RustcDecodable)]
struct SearchResult {
    crates: Vec<Crate>,
    meta: Meta,
}

impl SearchResult {
    fn from_data(buf: &[u8]) -> Result<SearchResult, json::DecoderError> {
        str::from_utf8(buf)
            .map_err(|e| json::DecoderError::ApplicationError(format!("{}", e)))
            .and_then(json::decode)
    }
}

pub fn handle_interactive_search(_args: &clap::ArgMatches) {
    let stdin = io::stdin();
    let mut stdout = ok_or_exit(io::stdout().into_raw_mode());
    ok_or_exit(write!(stdout, "{}{}", cursor::Goto(1, 1), clear::All));
    let mut term = String::new();
    let (sender, receiver) = mpsc::channel(4);
    let pool = CpuPool::new(1);
    let t = thread::spawn(|| {
        let mut reactor = ok_or_exit(Core::new());
        let session = Session::new(reactor.handle());
        let search_terms = receiver.and_then(|term| {
                ok_or_exit(write!(io::stdout(),
                                  "{}{}{}searching {}",
                                  cursor::Hide,
                                  cursor::Goto(1, 2),
                                  clear::CurrentLine,
                                  term));
                let mut req = Easy::new();
                ok_or_exit(req.get(true));
                let url = format!("https://crates.io/api/v1/crates?page=1&per_page=10&q={}&sort=",
                                  percent_encode(String::as_bytes(&term), DEFAULT_ENCODE_SET)
                                      .collect::<String>());
                ok_or_exit(req.url(&url));
                let buf = Arc::new(Mutex::new(Vec::new()));
                let buf_handle = buf.clone();
                ok_or_exit(req.write_function(move |data| {
                    write!(io::stdout(),
                           "{}{} {} bytes received",
                           cursor::Hide,
                           cursor::Goto(1, 3),
                           data.len())
                        .unwrap();
                    buf_handle.lock().unwrap().extend_from_slice(data);
                    Ok(data.len())
                }));
                session.perform(req).map_err(|_| ()).map(move |r| {
                    let result = SearchResult::from_data(&buf.lock().unwrap());
                    (r, result, term)
                })
            })
            .for_each(|(mut response, result, term)| {
                let result: SearchResult = ok_or_exit(result);
                ok_or_exit(write!(io::stdout(),
                                  "{}{}{}    {} found for '{}' done !",
                                  cursor::Hide,
                                  cursor::Goto(1, 2),
                                  clear::CurrentLine,
                                  result.meta.total,
                                  term));
                io::stdout().flush().ok();
                Ok(())
            });
        println!("Running reactor");
        reactor.run(search_terms).ok();
        println!("done running reactor - shutting down");
    });

    let mut ongoing_search = None;
    for k in stdin.keys() {
        match ok_or_exit(k) {
            Key::Char(c) => {
                term.push(c);
            }
            Key::Backspace => {
                term.pop();
            }
            Key::Esc => {
                break;
            }
            _ => println!("unsupported!"),
        }
        ok_or_exit(write!(stdout,
                          "{}{}{}crates.io search: {}",
                          cursor::Show,
                          cursor::Goto(1, 1),
                          clear::CurrentLine,
                          term));
        stdout.flush().ok();
        ongoing_search = Some(pool.spawn(sender.clone().send(term.clone())));
    }
    drop(ongoing_search);
    drop(sender);
    t.join().unwrap();
}
