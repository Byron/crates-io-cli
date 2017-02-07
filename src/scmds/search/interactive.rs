use super::structs::{State, Indexed, Command, SearchResult};
use super::error::Error;
use clap;
use open;
use urlencoding;
use std::cmp::max;
use std::str;
use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;
use std::sync::atomic::{Ordering, AtomicUsize};
use std::sync::{Mutex, Arc};
use std::io::{self, Write};
use std::fmt::Display;
use std::thread;
use termion::event::Key;
use termion::raw::IntoRawMode;
use termion::input::TermRead;
use termion::{cursor, clear};
use tokio_core::reactor::{Timeout, Handle, Core};
use futures::{self, Sink, Stream, Future};
use futures::future::BoxFuture;
use futures::sync::mpsc;
use tokio_curl::Session;

use utils::{paged_crates_io_remote_call, CallResult, CallMetaData, DropOutdated, DroppedOrError,
            Dimension};

const INFO_LINE: cursor::Goto = cursor::Goto(1, 2);
const CONTENT_LINE: cursor::Goto = cursor::Goto(1, 3);
const NON_CONTENT_LINES: u16 = 2;

fn search_result_from_callresult(c: CallResult) -> Result<SearchResult, Error> {
    let (buf, _) = c;
    let buf_slice = buf.lock().unwrap();
    SearchResult::from_data(&buf_slice, dimension()).map_err(|e| {
        write!(io::stderr(),
               "Json decoder failed\n{}\n",
               String::from_utf8_lossy(&buf_slice))
            .ok();
        Error::Decode(e)
    })
}

fn merge(mut r: SearchResult, c: CallResult) -> Result<SearchResult, Error> {
    search_result_from_callresult(c).map(|mut res| {
        r.crates.append(&mut res.crates);
        r
    })
}

fn extract(c: CallResult) -> Result<(CallMetaData, SearchResult), Error> {
    search_result_from_callresult(c).map(|res| {
        (CallMetaData {
             total: res.meta.total,
             items: res.crates.len() as u32,
         },
         res)
    })
}

fn dimension() -> Dimension {
    Dimension::default().loose_heigth(NON_CONTENT_LINES)
}

use super::structs::Command::*;
use super::structs::Mode::*;

enum ReducerDo {
    Nothing,
    Clear,
    ShowLast,
    Show(SearchResult),
    DrawIndices,
    Open { force: bool, number: usize },
}

fn setup_future(cmd: Command,
                session: Arc<Mutex<Session>>,
                handle: &Handle,
                version: &Arc<AtomicUsize>)
                -> BoxFuture<ReducerDo, Error> {
    match cmd {
        Clear => futures::finished(ReducerDo::Clear).boxed(),
        Open { force, number } => {
            futures::finished(ReducerDo::Open {
                    force: force,
                    number: number,
                })
                .boxed()
        }
        DrawIndices => futures::finished(ReducerDo::DrawIndices).boxed(),
        ShowLast => futures::finished(ReducerDo::ShowLast).boxed(),
        Search(term) => {
            let version = {
                version.fetch_add(1, Ordering::SeqCst);
                version.clone()
            };

            let dim = dimension();
            let url = format!("https://crates.io/api/v1/crates?page=1&per_page={}&q={}&sort=",
                              max(100, dim.height),
                              urlencoding::encode(&term));
            let req = paged_crates_io_remote_call(&url,
                                                  Some(dim.height as u32),
                                                  session.clone(),
                                                  merge,
                                                  extract);
            info(&"searching ...");
            let default_timeout: Duration = Duration::from_millis(5000);
            let timeout = Timeout::new(default_timeout.clone(), handle)
                .map(Future::boxed)
                .unwrap_or_else(|_| futures::empty().boxed())
                .map_err(Error::Timeout)
                .map(move |_| {
                    info(&format!("Timeout occurred after {:?} - request dropped. Keep typing \
                                   to try again.",
                                  default_timeout));
                    ReducerDo::Nothing
                });
            let req = req.map_err(move |e| {
                    info(&format!("Request to {} failed with error: '{}'", url, e));
                    e.into()
                })
                .map(move |mut result| {
                    result.meta.term = Some(term);
                    ReducerDo::Show(result)
                });

            let req = req.select(timeout)
                .then(|res| {
                    Ok(match res {
                        Ok((do_nothing @ ReducerDo::Nothing, pending_request)) => {
                            drop(pending_request);
                            do_nothing
                        }
                        Ok((result, _timeout)) => result,
                        Err(_) => ReducerDo::Nothing,
                    })
                })
                .boxed();
            DropOutdated::with_version(req, version.clone())
                .or_else(|e| match e {
                    DroppedOrError::Dropped => Ok(ReducerDo::Nothing),
                    DroppedOrError::Err(e) => Err(e),
                })
                .boxed()
        }
    }
}

fn handle_future_result(cmd: ReducerDo,
                        current_result: Option<&SearchResult>)
                        -> Option<Option<SearchResult>> {
    use self::ReducerDo::*;
    let mut res = None;
    match (cmd, current_result) {
        (Nothing, _) => {}
        (DrawIndices, None) => {
            info(&"There is nothing to open - conduct a search first.");
        }
        (DrawIndices, Some(ref search)) => {
            info(&"(<ESC> to quit, Ctrl+o to cancel, <enter> to confirm) Type the number of the \
                   crate to open.");
            write!(io::stdout(),
                   "{goto}{}",
                   Indexed(search),
                   goto = CONTENT_LINE)
                .ok();
        }
        (Open { .. }, None) => {
            info(&"There is nothing to open - conduct a search first");
        }
        (Open { force, number }, Some(search)) => {
            match search.crates.get(number) {
                Some(c1) => {
                    if number == 0 || search.crates.get(number * 10).is_none() || force {
                        let url = format!("https://crates.io/crates/{n}/{v}",
                                          n = c1.name,
                                          v = c1.max_version);
                        if let Err(e) = open::that(url) {
                            info(&e);
                        }
                    } else {
                        info(&format!("Hit <enter> to open crate #{} or keep typing ...", number));
                    }
                }
                None => {
                    info(&format!("No crate #{}! Try using <backspace> ...", number));
                }
            }
        }
        (Clear, _) => {
            usage();
            let empty_search = SearchResult::with_dimension(dimension());
            write!(io::stdout(), "{goto}{}", empty_search, goto = CONTENT_LINE).ok();
            res = Some(None);
        }
        (ShowLast, None) => {
            info(&"There is no previous result - conduct a search first.");
        }
        (ShowLast, Some(ref search)) => {
            write!(io::stdout(), "{goto}{}", search, goto = CONTENT_LINE).ok();
        }
        (Show(result), last_search) => {
            info(&format!("{} results for '{}' in total, showing {} max",
                          result.meta.total,
                          result.meta.term.as_ref().map(|s| s.as_str()).unwrap_or(""),
                          result.meta
                              .dimension
                              .as_ref()
                              .expect("dimension to be set")
                              .height));
            if result.crates.is_empty() {
                let last = usage();
                let suffix = last_search.and_then(|r| r.meta.term.as_ref())
                    .map(|term| format!("Showing results for '{}'", term))
                    .unwrap_or_else(String::new);
                write!(io::stdout(),
                       "{gotolast} - nothing found.{suffix}",
                       suffix = suffix,
                       gotolast = cursor::Goto(last as u16, INFO_LINE.1))
                    .ok();
            } else {
                write!(io::stdout(), "{goto}{}", result, goto = CONTENT_LINE).ok();
                res = Some(Some(result));
            }
        }
    }
    io::stdout().flush().ok();
    res
}

enum LoopControl {
    ShouldBreak,
    ShouldKeepGoing,
}

fn handle_key(k: Key,
              sender: mpsc::Sender<Command>,
              state: &mut State)
              -> Result<LoopControl, Error> {
    let (mut force_open, mut show_last_search) = (false, false);
    match k {
        Key::Char('\n') => {
            match state.mode {
                Searching => state.term.clear(),
                Opening => {
                    force_open = true;
                    state.mode = Opening;
                }
            }
        }
        Key::Char(c) => {
            match state.mode {
                Searching => {
                    if !is_special(c) {
                        state.term.push(c)
                    }
                }
                Opening => {
                    match c {
                        '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                            state.number.push(c)
                        }
                        _ => {
                            info(&format!("Please enter digits from 0-9"));
                            return Ok(LoopControl::ShouldKeepGoing);
                        }
                    }
                }
            }
        }
        Key::Backspace => {
            match state.mode {
                    Searching => &mut state.term,
                    Opening => &mut state.number,
                }
                .pop();
        }
        Key::Ctrl('o') => {
            state.mode = match state.mode {
                Searching => Opening,
                Opening => {
                    state.number.clear();
                    show_last_search = true;
                    Searching
                }
            };
        }
        Key::Esc | Key::Ctrl('c') => {
            return Ok(LoopControl::ShouldBreak);
        }
        key @ _ => {
            info(&format!("unsupported key sequence: {:?}", key));
            return Ok(LoopControl::ShouldKeepGoing);
        }
    }
    promptf(&state);
    let cmd = match state.mode {
        Searching => {
            if state.term.is_empty() {
                Clear
            } else {
                match show_last_search {
                    true => ShowLast,
                    false => Search(state.term.clone()),
                }
            }
        }
        Opening if state.number.len() > 0 => {
            Open {
                force: force_open,
                number: match state.number.parse() {
                    Ok(n) => n,
                    Err(e) => {
                        info(&e);
                        state.number.clear();
                        return Ok(LoopControl::ShouldKeepGoing);
                    }
                },
            }
        }
        Opening => DrawIndices,
    };
    sender.send(cmd).wait().map_err(Error::SendCommand)?;
    return Ok(LoopControl::ShouldKeepGoing);
}

pub fn handle_interactive_search(_args: &clap::ArgMatches) -> Result<(), Error> {
    let stdin = io::stdin();
    let mut stdout = io::stdout().into_raw_mode()?;
    let mut state = State::default();

    write!(stdout, "{}{}", cursor::Goto(1, 1), clear::All).map_err(Error::FirstIo)?;
    promptf(&state);
    usage();

    let (sender, receiver) = mpsc::channel(10);
    let t = thread::spawn(|| {
        let mut reactor = match Core::new() {
            Err(e) => return Err(Error::ReactorInit(e)),
            Ok(r) => r,
        };
        let session = Arc::new(Mutex::new(Session::new(reactor.handle())));
        let handle = reactor.handle();
        let version = Arc::new(AtomicUsize::new(0));
        let current_result = Rc::new(RefCell::new(None));

        let commands = receiver.and_then(|cmd: Command| {
                let cr = current_result.clone();
                let spawnable = setup_future(cmd, session.clone(), &handle, &version)
                    .then(|r| {
                        match r {
                            Ok(r) => Ok(r),
                            Err(Error::Decode(_)) => Err(()), /*abort stream on decode error*/
                            Err(_) => Ok(ReducerDo::Nothing), /*ignore other errors*/
                        }
                    })
                    .and_then(move |result| {
                        let res = handle_future_result(result, cr.borrow().as_ref());
                        if let Some(next_result) = res {
                            *cr.borrow_mut() = next_result;
                        }
                        Ok(())
                    });
                handle.spawn(spawnable);
                Ok(())
            })
            .for_each(|_| Ok(()));
        reactor.run(commands).ok();
        Ok(())
    });

    for k in stdin.keys() {
        if let LoopControl::ShouldBreak = handle_key(k.map_err(Error::KeySequence)?,
                                                     sender.clone(),
                                                     &mut state)? {
            break;
        }
    }
    drop(sender);
    let res = t.join().map_err(|_| Error::ThreadPanic).and_then(|r| r);
    reset_terminal();
    res
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

fn promptf(state: &State) {
    write!(io::stdout(),
           "{show}{goto}{clear} {mode}: {}",
           state.prompt(),
           mode = state.mode,
           show = cursor::Show,
           goto = cursor::Goto(1, 1),
           clear = clear::CurrentLine)
        .ok();
    io::stdout().flush().ok();
}

fn is_special(c: char) -> bool {
    c == '\t'
}
