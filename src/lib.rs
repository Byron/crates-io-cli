#[macro_use]
extern crate clap;
#[macro_use]
extern crate prettytable;
extern crate rustc_serialize;
extern crate crates_index_diff;
extern crate futures;
extern crate futures_cpupool;
extern crate curl;
extern crate tokio_core;
extern crate tokio_curl;
#[cfg(unix)]
extern crate termion;
extern crate open;
#[macro_use]
extern crate quick_error;
extern crate git2;
extern crate urlencoding;


mod utils;
mod scmds;
mod structs;

pub use utils::ok_or_exit;
pub use scmds::{handle_interactive_search, handle_recent_changes, handle_list, by_user, OutputKind};
