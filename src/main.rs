#![deny(unsafe_code)]

#[cfg_attr(any(feature = "recent-changes", feature = "list"), macro_use)]
extern crate prettytable;
#[cfg_attr(
    any(feature = "list", feature = "recent-changes", feature = "search"),
    macro_use
)]
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

use crate::args::Parsed;

fn main() {
    env_logger::init();
    use args::SubCommands::*;
    use clap::Parser;
    let args: Parsed = args::Parsed::parse();

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
        #[cfg(feature = "mine")]
        Some(Criner(args)) => ok_or_exit(criner_cli::run_blocking(args)),
        None =>
        {
            #[cfg(feature = "search")]
            ok_or_exit(handle_interactive_search())
        }
    }
}
