extern crate crates_index_diff;
extern crate clap;

use std::path::PathBuf;
use std::env;
use std::error::Error;
use std::io::Write;
use std::fmt::{self, Formatter, Display};

use clap::{Arg, SubCommand, App};
use crates_index_diff::{CrateVersion, Index};

const CHANGES_SUBCOMMAND_DESCRIPTION: &'static str = r##"
The output of this command is based on the state of the current crates.io repository clone.
It will remember the last result, so that the next invocation might yield different (or no)
changed crates at all.
Please note that the first query is likely to yield more than 40000 results!
The first invocation may be slow as it might have to clone the crates.io index.
"##;

struct ForHumans<'a>(&'a CrateVersion);

impl<'a> Display for ForHumans<'a> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.0.name, self.0.version, self.0.kind)
    }
}

fn default_repository_dir() -> PathBuf {
    let mut p = env::temp_dir();
    p.push("crates-io-bare-clone_for-cli");
    p
}

fn ok_or_exit<T, E>(result: Result<T, E>) -> T
    where E: Error
{
    match result {
        Ok(v) => v,
        Err(err) => {
            println!("{}", err);
            std::process::exit(2);
        }
    }
}

fn main() {
    let temp_dir = default_repository_dir();
    let temp_dir_str = temp_dir.to_string_lossy();
    let app = App::new("crates.io interface")
        .version("1.0")
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
            .after_help(CHANGES_SUBCOMMAND_DESCRIPTION));

    let matches = app.get_matches();
    let repo_path = matches.value_of("repository").expect("defaut to be set");

    match matches.subcommand() {
        ("recent-changes", Some(_)) => {
            let index = ok_or_exit(Index::from_path_or_cloned(repo_path));
            let stdout = std::io::stdout();
            let mut channel = stdout.lock();
            for version in ok_or_exit(index.fetch_changes()) {
                writeln!(channel, "{}", ForHumans(&version)).ok();
            }
        }
        _ => {
            print!("{}\n", matches.usage());
            std::process::exit(1);
        }
    }
}
