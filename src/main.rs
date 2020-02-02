#![allow(unused_imports, dead_code)]

#[macro_use]
extern crate clap;
#[macro_use]
extern crate prettytable;
#[macro_use]
extern crate quick_error;

mod args;
mod scmds;
mod structs;
mod utils;

use scmds::handle_recent_changes;
use structopt::StructOpt;
use utils::ok_or_exit;

use crate::args::Parsed;

fn main() {
    use args::SubCommands::*;
    let args: Parsed = args::Parsed::from_args();

    match args.sub.unwrap_or(args::SubCommands::Search) {
        RecentChanges {
            repository,
            output_format,
        } => ok_or_exit(handle_recent_changes(repository, output_format)),
        _ => unimplemented!(),
    }

    //    let matches = app.get_matches();
    //    match matches.subcommand() {
    //        ("recent-changes", Some(args)) => ok_or_exit(handle_recent_changes(args)),
    //        ("search", Some(_)) => ok_or_exit(handle_interactive_search()),
    //        ("list", Some(list_args)) => {
    //            let (subcommand_handler, subcommand_args) = match list_args.subcommand() {
    //                ("by-user", Some(args)) => (by_user, args),
    //                _ => invalid_subcommand(list_args),
    //            };
    //            ok_or_exit(handle_list(list_args, subcommand_args, subcommand_handler));
    //        }
    //        _ => ok_or_exit(handle_interactive_search()),
    //    }
}
