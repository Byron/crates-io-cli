extern crate curl;
extern crate termion;
extern crate futures;
extern crate tokio_core;
extern crate tokio_curl;

use clap;
use std::thread;
use std::sync::mpsc::channel;
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
    let (mut sender, receiver) = mpsc::channel(4);
    thread::spawn(|| {
        let reactor = ok_or_exit(Core::new());
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
                        io::stdout().write_all(data).unwrap();
                        Ok(data.len())
                    })
                    .unwrap();
                session.perform(req)
            })
            .for_each(|response| {
                ok_or_exit(write!(io::stdout(),
                                  "{}{}{}    {} done !",
                                  cursor::Hide,
                                  cursor::Goto(1, 2),
                                  clear::CurrentLine,
                                  term));
                io::stdout().flush().ok();
                Ok(())
            });
        reactor.run(search_terms);
    });

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
        sender.send(term.clone());
    }
}
