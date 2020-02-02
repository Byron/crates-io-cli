#![allow(unused_imports, dead_code)]
// TODO: remove these allow attributes

#[macro_use]
extern crate clap;
#[macro_use]
extern crate prettytable;
#[macro_use]
extern crate quick_error;

mod args;
mod error;
#[cfg(feature = "list")]
mod http_utils;
mod scmds;
mod structs;

use error::ok_or_exit;
#[cfg(feature = "recent-changes")]
use scmds::handle_recent_changes;
#[cfg(feature = "list")]
use scmds::{by_user, handle_list};
use structopt::StructOpt;

use crate::args::Parsed;

fn main() {
    use args::SubCommands::*;
    let args: Parsed = args::Parsed::from_args();

    match args.sub {
        #[cfg(not(feature = "list"))]
        Some(_) => {}
        #[cfg(feature = "recent-changes")]
        Some(RecentChanges {
            repository,
            output_format,
        }) => ok_or_exit(handle_recent_changes(repository, output_format)),
        #[cfg(feature = "list")]
        Some(List { cmd, output_format }) => {
            use args::ListCmd::*;
            ok_or_exit(match cmd {
                ByUser { id } => handle_list(output_format, move |session| by_user(id, session)),
            })
        }
        None => {}
    }

    //    let matches = app.get_matches();
    //    match matches.subcommand() {
    //        ("search", Some(_)) => ok_or_exit(handle_interactive_search()),
    //        _ => ok_or_exit(handle_interactive_search()),
    //    }
}
