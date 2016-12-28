extern crate crates_index_diff;
#[macro_use]
extern crate clap;
extern crate rustc_serialize;
extern crate tokio_core;
extern crate futures;
extern crate futures_cpupool;

use futures_cpupool::CpuPool;
use futures::Future;
use std::path::PathBuf;
use std::env;
use std::time::Duration;
use std::error::Error;
use std::io::{self, Write};
use std::fmt::{self, Formatter, Display};
use rustc_serialize::Encodable;
use rustc_serialize::json;
use tokio_core::reactor::{Timeout, Core};

use clap::{Arg, SubCommand, App};
use crates_index_diff::{CrateVersion, Index};

const CHANGES_SUBCOMMAND_DESCRIPTION: &'static str = r##"
The output of this command is based on the state of the current crates.io repository clone.
It will remember the last result, so that the next invocation might yield different (or no)
changed crates at all.
Please note that the first query is likely to yield more than 40000 results!
The first invocation may be slow as it might have to clone the crates.io index.
"##;

arg_enum!{
    #[allow(non_camel_case_types)]
    #[derive(Debug)]
    pub enum OutputKind {
        human,
        json
    }
}

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

fn handle_recent_changes(repo_path: &str, args: &clap::ArgMatches) {
    let mut reactor = ok_or_exit(Core::new());
    let handle: tokio_core::reactor::Handle = reactor.handle();
    let timeout: Timeout = ok_or_exit(Timeout::new(Duration::from_secs(3), &handle));
    let pool = CpuPool::new(1);

    let output_kind: OutputKind =
        args.value_of("format").expect("default to be set").parse().expect("clap to work");
    let owned_repo_path = repo_path.to_owned();

    let computation = pool.spawn_fn(move || {
        ok_or_exit(std::fs::create_dir_all(&owned_repo_path));
        let index = ok_or_exit(Index::from_path_or_cloned(owned_repo_path));
        let stdout = io::stdout();
        let mut channel = stdout.lock();
        let changes = ok_or_exit(index.fetch_changes());

        match output_kind {
            OutputKind::human => {
                for version in changes {
                    writeln!(channel, "{}", ForHumans(&version)).ok();
                }
            }
            OutputKind::json => {
                let mut buf = String::with_capacity(256);
                for version in changes {
                    buf.clear();
                    // unfortunately io::Write cannot be used directly, the encoder needs fmt::Write
                    // To allow us reusing the buffer, we need to restrict its lifetime.
                    if {
                            let mut encoder = json::Encoder::new(&mut buf);
                            version.encode(&mut encoder)
                        }
                        .is_ok() {
                        writeln!(channel, "{}", buf).ok();
                    }
                }
            }
        }
        Ok(Ok(()))
    });
    let owned_repo_path = repo_path.to_owned();
    let timeout = timeout.and_then(move |_| {
        writeln!(std::io::stderr(),
                 "Please wait while we check out or fetch the crates.io index at \
                  '{path}'",
                 path = owned_repo_path)
            .ok();
        Ok(Err(()))
    });
    let computation = computation.select(timeout).then(|res| {
        match res {
            Ok((Ok(_), _)) => Ok(()),
            Ok((Err(_), computation)) => computation.wait().map(|_| ()),
            Err((e, _drop_timeout)) => Err(e),
        }
    });
    ok_or_exit(reactor.run(computation));
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
            .arg(Arg::with_name("format")
                .short("o")
                .long("output")
                .required(false)
                .takes_value(true)
                .default_value(&human_output)
                .possible_values(&OutputKind::variants())
                .help("The type of output to produce."))
            .after_help(CHANGES_SUBCOMMAND_DESCRIPTION));

    let matches = app.get_matches();
    let repo_path = matches.value_of("repository").expect("default to be set");

    match matches.subcommand() {
        ("recent-changes", Some(args)) => handle_recent_changes(repo_path, args),
        _ => {
            print!("{}\n", matches.usage());
            std::process::exit(1);
        }
    }
}
