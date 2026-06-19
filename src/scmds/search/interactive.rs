use super::error::Error;
use super::structs::{Command, Dimension, Indexed, SearchResult, State};
use open;
use std::{
    cmp::max,
    fmt::Display,
    io::{self, Write},
    sync::mpsc,
    thread,
};
use termion::{clear, cursor, event::Key, input::TermRead, raw::IntoRawMode};
use urlencoding;

use crate::http_utils::{
    CallMetaData, CallResult, CancelFlag, new_cancel_flag, paged_crates_io_remote_call,
};

const INFO_LINE: cursor::Goto = cursor::Goto(1, 2);
const CONTENT_LINE: cursor::Goto = cursor::Goto(1, 3);
const NON_CONTENT_LINES: u16 = 2;

fn search_result_from_callresult(c: CallResult) -> Result<SearchResult, Error> {
    let (buf, _) = c;
    let buf_slice = buf.lock().unwrap();
    SearchResult::from_data(&buf_slice, dimension()).map_err(|e| {
        write!(
            io::stderr(),
            "Json decoder failed\n{}\n",
            String::from_utf8_lossy(&buf_slice)
        )
        .ok();
        e.into()
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
        (
            CallMetaData {
                total: res.meta.total,
                items: res.crates.len() as u32,
            },
            res,
        )
    })
}

fn dimension() -> Dimension {
    Dimension::default().loose_heigth(NON_CONTENT_LINES)
}

use super::structs::Command::*;
use super::structs::Mode::*;

enum ReducerDo {
    Clear,
    ShowLast,
    Show(SearchResult),
    DrawIndices,
    Open { force: bool, number: usize },
}

fn handle_command(cmd: Command, cancel: Option<CancelFlag>) -> Result<ReducerDo, Error> {
    match cmd {
        Clear => Ok(ReducerDo::Clear),
        Open { force, number } => Ok(ReducerDo::Open {
            force: force,
            number: number,
        }),
        DrawIndices => Ok(ReducerDo::DrawIndices),
        ShowLast => Ok(ReducerDo::ShowLast),
        Search(term) => {
            let dim = dimension();
            let url = format!(
                "https://crates.io/api/v1/crates?page=1&per_page={}&q={}&sort=",
                max(100, dim.height),
                urlencoding::encode(&term)
            );
            let req =
                paged_crates_io_remote_call(&url, Some(dim.height as u32), merge, extract, cancel);
            req.map(|mut result| {
                result.meta.term = Some(term);
                ReducerDo::Show(result)
            })
            .map_err(Into::into)
        }
    }
}

fn handle_future_result(
    cmd: ReducerDo,
    current_result: Option<&SearchResult>,
) -> Option<Option<SearchResult>> {
    use self::ReducerDo::*;
    let mut res = None;
    match (cmd, current_result) {
        (DrawIndices, None) => {
            info(&"There is nothing to open - conduct a search first.");
        }
        (DrawIndices, Some(ref search)) => {
            info(
                &"(<ESC> to quit, Ctrl+o to cancel, <enter> to confirm) Type the number of the \
                  crate to open.",
            );
            write!(
                io::stdout(),
                "{goto}{}",
                Indexed(search),
                goto = CONTENT_LINE
            )
            .ok();
        }
        (Open { .. }, None) => {
            info(&"There is nothing to open - conduct a search first");
        }
        (Open { force, number }, Some(search)) => match search.crates.get(number) {
            Some(c1) => {
                if number == 0 || search.crates.get(number * 10).is_none() || force {
                    let url = format!(
                        "https://crates.io/crates/{n}/{v}",
                        n = c1.name,
                        v = c1.max_version
                    );
                    if let Err(e) = open::that(url) {
                        info(&e);
                    }
                } else {
                    info(&format!(
                        "Hit <enter> to open crate #{} or keep typing ...",
                        number
                    ));
                }
            }
            None => {
                info(&format!("No crate #{}! Try using <backspace> ...", number));
            }
        },
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
            info(&format!(
                "{} results for '{}' in total, showing {} max",
                result.meta.total,
                result.meta.term.as_ref().map(|s| s.as_str()).unwrap_or(""),
                result
                    .meta
                    .dimension
                    .as_ref()
                    .expect("dimension to be set")
                    .height
            ));
            if result.crates.is_empty() {
                let last = usage();
                let suffix = last_search
                    .and_then(|r| r.meta.term.as_ref())
                    .map(|term| format!("Showing results for '{}'", term))
                    .unwrap_or_else(String::new);
                write!(
                    io::stdout(),
                    "{gotolast} - nothing found.{suffix}",
                    suffix = suffix,
                    gotolast = cursor::Goto(last as u16, INFO_LINE.1)
                )
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

enum WorkerEvent {
    Command(Command),
    SearchFinished {
        version: usize,
        result: Result<ReducerDo, Error>,
    },
    Shutdown,
}

fn apply_worker_result(
    result: Result<ReducerDo, Error>,
    current_result: &mut Option<SearchResult>,
) {
    match result {
        Ok(result) => {
            let res = handle_future_result(result, current_result.as_ref());
            if let Some(next_result) = res {
                *current_result = next_result;
            }
        }
        Err(e) => {
            info(&e);
        }
    }
}

fn spawn_search(
    term: String,
    version: usize,
    sender: mpsc::Sender<WorkerEvent>,
    cancel: CancelFlag,
) {
    thread::spawn(move || {
        let result = handle_command(Search(term), Some(cancel));
        sender
            .send(WorkerEvent::SearchFinished { version, result })
            .ok();
    });
}

fn handle_key(
    k: Key,
    sender: &mpsc::Sender<WorkerEvent>,
    state: &mut State,
) -> Result<LoopControl, Error> {
    let (mut force_open, mut show_last_search) = (false, false);
    match k {
        Key::Char('\n') => match state.mode {
            Searching => state.term.clear(),
            Opening => {
                force_open = true;
                state.mode = Opening;
            }
        },
        Key::Char(c) => match state.mode {
            Searching => {
                if !is_special(c) {
                    state.term.push(c)
                }
            }
            Opening => match c {
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => state.number.push(c),
                _ => {
                    info(&format!("Please enter digits from 0-9"));
                    return Ok(LoopControl::ShouldKeepGoing);
                }
            },
        },
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
        Opening if state.number.len() > 0 => Open {
            force: force_open,
            number: match state.number.parse() {
                Ok(n) => n,
                Err(e) => {
                    info(&e);
                    state.number.clear();
                    return Ok(LoopControl::ShouldKeepGoing);
                }
            },
        },
        Opening => DrawIndices,
    };
    sender
        .send(WorkerEvent::Command(cmd))
        .map_err(|_| Error::SendCommand)?;
    Ok(LoopControl::ShouldKeepGoing)
}

pub fn handle_interactive_search() -> Result<(), Error> {
    let stdin = io::stdin();
    let mut stdout = io::stdout().into_raw_mode()?;
    let mut state = State::default();

    write!(stdout, "{}{}", cursor::Goto(1, 1), clear::All).map_err(Error::FirstIo)?;
    promptf(&state);
    usage();

    let (sender, receiver) = mpsc::channel();
    let worker_sender = sender.clone();
    let t = thread::spawn(move || {
        let mut current_result = None;
        let mut search_version = 0;
        let mut current_search_cancel: Option<CancelFlag> = None;

        while let Ok(event) = receiver.recv() {
            match event {
                WorkerEvent::Command(Search(term)) => {
                    search_version += 1;
                    if let Some(cancel) = current_search_cancel.take() {
                        cancel.store(true, std::sync::atomic::Ordering::Relaxed);
                    }
                    let cancel = new_cancel_flag();
                    current_search_cancel = Some(cancel.clone());
                    info(&"searching ...");
                    spawn_search(term, search_version, worker_sender.clone(), cancel);
                }
                WorkerEvent::Command(Clear) => {
                    search_version += 1;
                    if let Some(cancel) = current_search_cancel.take() {
                        cancel.store(true, std::sync::atomic::Ordering::Relaxed);
                    }
                    apply_worker_result(handle_command(Clear, None), &mut current_result);
                }
                WorkerEvent::Command(cmd) => {
                    apply_worker_result(handle_command(cmd, None), &mut current_result);
                }
                WorkerEvent::SearchFinished { version, result } => {
                    if version == search_version {
                        current_search_cancel = None;
                        apply_worker_result(result, &mut current_result);
                    }
                }
                WorkerEvent::Shutdown => {
                    if let Some(cancel) = current_search_cancel.take() {
                        cancel.store(true, std::sync::atomic::Ordering::Relaxed);
                    }
                    break;
                }
            }
        }

        Ok(())
    });

    for k in stdin.keys() {
        match handle_key(k.map_err(Error::KeySequence)?, &sender, &mut state)? {
            LoopControl::ShouldBreak => break,
            LoopControl::ShouldKeepGoing => {}
        }
    }
    sender.send(WorkerEvent::Shutdown).ok();
    let res = t.join().map_err(|_| Error::ThreadPanic).and_then(|r| r);
    reset_terminal();
    res
}

fn reset_terminal() {
    write!(
        io::stdout(),
        "{}{}{}",
        cursor::Goto(1, 1),
        cursor::Show,
        clear::All
    )
    .ok();
}

fn usage() -> usize {
    info(&"(<ESC> to quit, <enter> to clear, Ctrl+o to open) Please enter your search term.")
}

fn info(item: &dyn Display) -> usize {
    let buf = format!("{}", item);
    write!(
        io::stdout(),
        "{hide}{goto}{clear}{}",
        buf,
        hide = cursor::Hide,
        goto = INFO_LINE,
        clear = clear::CurrentLine
    )
    .ok();
    io::stdout().flush().ok();
    buf.len()
}

fn promptf(state: &State) {
    write!(
        io::stdout(),
        "{show}{goto}{clear} {mode}: {}",
        state.prompt(),
        mode = state.mode,
        show = cursor::Show,
        goto = cursor::Goto(1, 1),
        clear = clear::CurrentLine
    )
    .ok();
    io::stdout().flush().ok();
}

fn is_special(c: char) -> bool {
    c == '\t'
}
