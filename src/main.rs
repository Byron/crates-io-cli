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


mod utils;
mod scmds;

use scmds::{handle_interactive_search, handle_recent_changes, OutputKind};
use std::env;
use std::path::PathBuf;
use clap::{Arg, SubCommand, App};

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

fn main() {
    let temp_dir = default_repository_dir();
    let temp_dir_str = temp_dir.to_string_lossy();
    let human_output = format!("{}", OutputKind::human);
    let app = App::new("crates.io interface")
        .version(crate_version!())
        .author("Sebastian Thiel <byronimo@gmail.com>")
        .about("Interact with the https://crates.io index via the command-line")
        .arg(Arg::with_name("repository")
            .short("r")
            .long("repository")
            .value_name("REPO")
            .help("Path to the possibly existing crates.io repository clone.")
            .default_value(&temp_dir_str)
            .required(false)
            .takes_value(true))
        .subcommand(SubCommand::with_name("recent-changes")
            .about("show all recently changed crates")
            .display_order(1)
            .arg(Arg::with_name("format")
                .short("o")
                .long("output")
                .required(false)
                .takes_value(true)
                .default_value(&human_output)
                .possible_values(&OutputKind::variants())
                .help("The type of output to produce."))
            .after_help(CHANGES_SUBCOMMAND_DESCRIPTION))
        .subcommand(SubCommand::with_name("search")
            .display_order(2)
            .about("search crates interactively"));


    let matches = app.get_matches();
    let repo_path = matches.value_of("repository").expect("default to be set");

    match matches.subcommand() {
        ("recent-changes", Some(args)) => handle_recent_changes(repo_path, args),
        ("search", Some(args)) => handle_interactive_search(args),
        _ => {
            print!("{}\n", matches.usage());
            std::process::exit(1);
        }
    }
}
