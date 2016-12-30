use super::structs::SearchResult;
use utils::Dimension;
use clap;
use open;
use std::str;
use std::sync::{Mutex, Arc};
use std::thread;
use futures_cpupool::CpuPool;
use curl::easy::Easy;
use termion::event::Key;
use termion::raw::IntoRawMode;
use termion::input::TermRead;
use termion::clear;
use termion::cursor;
use tokio_core::reactor::Core;
use futures::{self, Sink, Stream, Future};
use futures::sync::mpsc;
use tokio_curl::Session;
use std::io::{self, Write};
use std::fmt::{self, Display};

use utils::ok_or_exit;

const INFO_LINE: cursor::Goto = cursor::Goto(1, 2);
const CONTENT_LINE: cursor::Goto = cursor::Goto(1, 3);
const NON_CONTENT_LINES: u16 = 2;

fn dimension() -> Dimension {
    Dimension::default().loose_heigth(NON_CONTENT_LINES)
}

#[derive(Clone)]
enum Command {
    Search(String),
    Open(bool, usize),
    DrawIndices,
    Clear,
}

#[derive(Clone, Copy)]
enum Mode {
    Searching,
    Opening(bool),
}

impl Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "{}",
               match *self {
                   Searching => "search",
                   Opening(_) => "open by number",
               })
    }
}

impl Default for Mode {
    fn default() -> Self {
        Searching
    }
}


use self::Command::*;
use self::Mode::*;

#[derive(Default)]
struct State {
    number: String,
    term: String,
    mode: Mode,
}

impl State {
    fn prompt(&self) -> &str {
        match self.mode {
            Searching => &self.term,
            Opening(_) => &self.number,
        }
    }
}

struct Indexed<'a>(&'a SearchResult);

impl<'a> Display for Indexed<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let dim = self.0.meta.dimension.clone().unwrap_or_default();
        let center = ((dim.width / 2) - 2) as u16;
        write!(f,
               "{hide}{align}",
               hide = cursor::Hide,
               align = cursor::Right(center))?;
        for i in (0..self.0.crates.len()).take(dim.height as usize) {
            let rendered = format!("-#{:3} #-", i);
            write!(f,
                   "{}{left}{down}",
                   rendered,
                   left = cursor::Left(rendered.len() as u16),
                   down = cursor::Down(1))?
        }
        Ok(())
    }
}

pub fn handle_interactive_search(_args: &clap::ArgMatches) {
    let stdin = io::stdin();
    let mut stdout = ok_or_exit(io::stdout().into_raw_mode());
    ok_or_exit(write!(stdout, "{}{}", cursor::Goto(1, 1), clear::All));
    let mut state = State::default();
    promptf(&state, &mut stdout);
    usage();

    let (sender, receiver) = mpsc::channel(10);

    let t = thread::spawn(|| {
        let mut reactor = ok_or_exit(Core::new());
        let session = Session::new(reactor.handle());
        let mut current_result = None;

        let commands = receiver.and_then(|cmd: Command| {
                match cmd.clone() {
                    Clear | Open(_, _) | DrawIndices => futures::finished((None, cmd)).boxed(),
                    Search(term) => {
                        let mut req = Easy::new();
                        let dim = dimension();
                        ok_or_exit(req.get(true));
                        let url = format!("https://crates.\
                                           io/api/v1/crates?page=1&per_page={}&q={}&sort=",
                                          dim.height,
                                          req.url_encode(String::as_bytes(&term)));
                        ok_or_exit(req.url(&url));
                        let buf = Arc::new(Mutex::new(Vec::new()));
                        let buf_handle = buf.clone();
                        ok_or_exit(req.write_function(move |data| {
                            buf_handle.lock().unwrap().extend_from_slice(data);
                            Ok(data.len())
                        }));
                        info(&"searching ...");
                        session.perform(req)
                            .map_err(|e| {
                                info(&e);
                                ()
                            })
                            .map(move |_response| {
                                let buf_slice = buf.lock().unwrap();
                                let result = SearchResult::from_data(&buf_slice, dim)
                                    .map_err(|e| {
                                        write!(io::stderr(),
                                               "{}\n",
                                               String::from_utf8_lossy(&buf_slice))
                                            .ok();
                                        e
                                    });
                                (Some(ok_or_exit(result)), cmd)
                            })
                            .boxed()
                    }
                }
            })
            .for_each(|(search, cmd)| {
                match cmd {
                    DrawIndices => {
                        match current_result {
                            Some(ref search) => {
                                info(&"(<ESC> to quit, Ctrl+o to cancel, <enter> to confirm) \
                                       Type the number of the crate to open.");
                                write!(io::stdout(),
                                       "{goto}{}",
                                       Indexed(search),
                                       goto = CONTENT_LINE)
                                    .ok();
                            }
                            None => {
                                info(&"There is nothing to open - conduct a search first.");
                            }
                        }
                    }
                    Open(force, number) => {
                        match current_result {
                            Some(ref search) => {
                                match search.crates.get(number) {
                                    Some(c1) => {
                                        if number == 0 ||
                                           search.crates.get(number * 10).is_none() ||
                                           force {
                                            let url = format!("https://crates.io/crates/{n}/{v}",
                                                              n = c1.name,
                                                              v = c1.max_version);
                                            if let Err(e) = open::that(url) {
                                                info(&e);
                                            }
                                        } else {
                                            info(&format!("Hit <enter> to open crate #{} or \
                                                           keep typing ...",
                                                          number));
                                        }
                                    }
                                    None => {
                                        info(&format!("No crate #{}! Try using <backspace> ...",
                                                      number));
                                    }
                                }
                            }
                            None => {
                                info(&"There is nothing to open - conduct a search first");
                            }
                        }
                    }
                    Clear => {
                        usage();
                        let empty_search = SearchResult::with_dimension(dimension());
                        write!(io::stdout(), "{goto}{}", empty_search, goto = CONTENT_LINE).ok();
                        current_result = Some(empty_search);
                    }
                    Search(_) => {
                        let search = search.expect("search result must be present");
                        info(&format!("{} results in total, showing {} max",
                                      search.meta.total,
                                      search.meta
                                          .dimension
                                          .as_ref()
                                          .expect("dimension to be set")
                                          .height));
                        if search.crates.is_empty() {
                            let last = usage();
                            write!(io::stdout(),
                                   "{gotolast} - 0 results found",
                                   gotolast = cursor::Goto(last as u16, INFO_LINE.1))
                                .ok();
                        } else {
                            write!(io::stdout(), "{goto}{}", search, goto = CONTENT_LINE).ok();
                            current_result = Some(search);
                        }
                    }
                }
                io::stdout().flush().ok();
                Ok(())
            });
        reactor.run(commands).ok();
        println!("Thread shutting down");
    });

    let mut ongoing_command = None;
    let pool = CpuPool::new(1);

    for k in stdin.keys() {
        match ok_or_exit(k) {
            Key::Char('\n') => {
                match state.mode {
                    Searching => state.term.clear(),
                    Opening(_) => {
                        state.mode = Opening(true);
                    }
                }
            }
            Key::Char(c) => {
                match state.mode {
                    Searching => state.term.push(c),
                    Opening(_) => {
                        match c {
                            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                                state.number.push(c)
                            }
                            _ => {
                                info(&format!("Please enter digits from 0-9"));
                            }
                        }
                    }
                }
            }
            Key::Backspace => {
                match state.mode {
                        Searching => &mut state.term,
                        Opening(_) => &mut state.number,
                    }
                    .pop();
            }
            Key::Ctrl('o') => {
                state.mode = match state.mode {
                    Searching => Opening(false),
                    Opening(_) => {
                        state.number.clear();
                        Searching
                    }
                };
            }
            Key::Esc => {
                break;
            }
            key @ _ => {
                info(&format!("unsupported key sequence: {:?}", key));
                continue;
            }
        }
        promptf(&state, &mut stdout);
        let cmd = match state.mode {
            Searching => {
                if state.term.is_empty() {
                    Clear
                } else {
                    Search(state.term.clone())
                }
            }
            Opening(force) if state.number.len() > 0 => {
                Open(force,
                     match state.number.parse() {
                         Ok(n) => n,
                         Err(e) => {
                             info(&e);
                             state.number.clear();
                             continue;
                         }
                     })
            }
            Opening(_) => DrawIndices,
        };
        ongoing_command = Some(pool.spawn(sender.clone().send(cmd.clone())));
    }
    drop(ongoing_command);
    drop(sender);
    t.join().unwrap();
    reset_terminal();
}

fn reset_terminal() {
    write!(io::stdout(),
           "{}{}{}",
           cursor::Goto(1, 1),
           cursor::Show,
           clear::All)
        .ok();
}

fn usage() -> usize {
    info(&"(<ESC> to quit, <enter> to clear, Ctrl+o to open) Please enter your search term.")
}

fn info(item: &Display) -> usize {
    let buf = format!("{}", item);
    write!(io::stdout(),
           "{hide}{goto}{clear}{}",
           buf,
           hide = cursor::Hide,
           goto = INFO_LINE,
           clear = clear::CurrentLine)
        .ok();
    io::stdout().flush().ok();
    buf.len()
}

fn promptf(state: &State, stdout: &mut io::Stdout) {
    write!(stdout,
           "{show}{goto}{clear} {mode}: {}",
           state.prompt(),
           mode = state.mode,
           show = cursor::Show,
           goto = cursor::Goto(1, 1),
           clear = clear::CurrentLine)
        .ok();
    stdout.flush().ok();
}
