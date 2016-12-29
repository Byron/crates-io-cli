extern crate curl;
extern crate termion;
extern crate futures;
extern crate tokio_core;
extern crate tokio_curl;
extern crate futures_cpupool;

use clap;
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
                req.get(true).unwrap();
                req.url("https://www.rust-lang.org").unwrap();
                req.write_function(|data| {
                        write!(io::stdout(), "{}{}", cursor::Hide, cursor::Goto(1, 2)).unwrap();
                        Ok(data.len())
                    })
                    .unwrap();
                session.perform(req).map_err(|_| ()).map(|r| (r, term))
            })
            .for_each(|(mut response, term)| {
                ok_or_exit(write!(io::stdout(),
                                  "{}{}{}    {} - {} done !",
                                  cursor::Hide,
                                  cursor::Goto(1, 2),
                                  clear::CurrentLine,
                                  response.response_code().unwrap(),
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
