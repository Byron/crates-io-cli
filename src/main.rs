#[macro_use]
extern crate clap;
#[macro_use]
extern crate prettytable;
#[macro_use]
extern crate quick_error;

mod args;
mod error;
#[cfg(any(feature = "list", feature = "search"))]
mod http_utils;
mod scmds;
mod structs;

use error::ok_or_exit;
#[cfg(feature = "search")]
use scmds::handle_interactive_search;
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
        #[cfg(feature = "search")]
        Some(Search) => ok_or_exit(handle_interactive_search()),
        None =>
        {
            #[cfg(feature = "search")]
            ok_or_exit(handle_interactive_search())
        }
    }
}
