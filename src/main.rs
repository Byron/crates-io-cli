#[macro_use]
extern crate clap;
#[cfg_attr(any(feature = "recent-changes", feature = "list"), macro_use)]
extern crate prettytable;
#[cfg_attr(
    any(feature = "list", feature = "recent-changes", feature = "search"),
    macro_use
)]
extern crate quick_error;

mod args;
mod error;
#[cfg(any(feature = "list", feature = "search"))]
mod http_utils;
mod scmds;
mod structs;

#[cfg(feature = "mine")]
use criner;
use error::ok_or_exit;
#[cfg(feature = "search")]
use scmds::handle_interactive_search;
#[cfg(feature = "recent-changes")]
use scmds::handle_recent_changes;
#[cfg(feature = "list")]
use scmds::{by_user, handle_list};
use structopt::StructOpt;

use crate::args::Parsed;
use std::ops::Add;

fn main() {
    env_logger::init();
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
        #[cfg(feature = "mine")]
        Some(Mine {
            repository,
            db_path,
            time_limit,
        }) => ok_or_exit(criner::run_blocking(
            db_path,
            repository
                .unwrap_or_else(|| std::env::temp_dir().join("criner-crates-io-bare-index.git")),
            time_limit.map(|d| std::time::SystemTime::now().add(*d)),
        )),
        None =>
        {
            #[cfg(feature = "search")]
            ok_or_exit(handle_interactive_search())
        }
    }
}
