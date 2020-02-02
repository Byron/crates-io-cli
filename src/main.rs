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

use scmds::{by_user, handle_interactive_search, handle_list, handle_recent_changes};
use structopt::StructOpt;
use structs::OutputKind;
use utils::ok_or_exit;

use clap::{AppSettings, Arg, SubCommand};
use std::{env, path::PathBuf};

const CHANGES_SUBCOMMAND_DESCRIPTION: &'static str = r##"
The output of this command is based on the state of the current crates.io repository clone.
It will remember the last result, so that the next invocation might yield different (or no)
changed crates at all.
Please note that the first query is likely to yield more than 40000 results!
The first invocation may be slow as it might have to clone the crates.io index.
"##;

fn default_repository_dir() -> PathBuf {
    let mut p = env::temp_dir();
    p.push("crates-io-bare-clone_for-cli");
    p
}

fn invalid_subcommand(matches: &clap::ArgMatches) -> ! {
    print!("{}\n", matches.usage());
    std::process::exit(1)
}

fn main() {
    let temp_dir = default_repository_dir();
    let _args = args::Parsed::from_args();
    let temp_dir_str = temp_dir.to_string_lossy();
    let human_output = format!("{}", OutputKind::human);
    let format_arg = Arg::with_name("format")
        .short("o")
        .long("output")
        .required(false)
        .takes_value(true)
        .default_value(&human_output)
        .possible_values(&OutputKind::variants())
        .help("The type of output to produce.");
    let app = app_from_crate!()
        .setting(AppSettings::ColorAuto)
        .setting(AppSettings::ColoredHelp)
        .subcommand(SubCommand::with_name("recent-changes")
            .about("show all recently changed crates")
            .display_order(1)
            .arg(Arg::with_name("repository")
                .short("r")
                .long("repository")
                .value_name("REPO")
                .help("Path to the possibly existing crates.io repository clone.")
                .default_value(&temp_dir_str)
                .required(false)
                .takes_value(true))
            .arg(format_arg.clone())
            .after_help(CHANGES_SUBCOMMAND_DESCRIPTION))
        .subcommand(SubCommand::with_name("search")
            .display_order(2)
            .about("search crates interactively"))
        .subcommand(SubCommand::with_name("list")
            .display_order(3)
            .arg(format_arg)
            .subcommand(SubCommand::with_name("by-user")
                .arg(Arg::with_name("user-id")
                    .required(true)
                    .takes_value(true)
                    .help("The numerical id of your user, e.g. 980. Currently there is no way \
                           to easily obtain it though, so you will have to debug actual \
                           crates.io calls in your browser - the /me response contains all \
                           user data. Use any string to receive *all* crates!"))
                .about("crates for the given username"))
            .about("list crates by a particular criterion"));

    let matches = app.get_matches();
    match matches.subcommand() {
        ("recent-changes", Some(args)) => ok_or_exit(handle_recent_changes(args)),
        ("search", Some(_)) => ok_or_exit(handle_interactive_search()),
        ("list", Some(list_args)) => {
            let (subcommand_handler, subcommand_args) = match list_args.subcommand() {
                ("by-user", Some(args)) => (by_user, args),
                _ => invalid_subcommand(list_args),
            };
            ok_or_exit(handle_list(list_args, subcommand_args, subcommand_handler));
        }
        _ => ok_or_exit(handle_interactive_search()),
    }
}
