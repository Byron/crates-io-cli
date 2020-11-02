#[cfg(feature = "mine")]
use criner_cli;

#[cfg(any(feature = "mine", feature = "recent-changes"))]
use std::path::PathBuf;
use clap::Clap;
use std::str::FromStr;

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum OutputKind {
    human,
    json
}

impl FromStr for OutputKind {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "human"|"Human" => OutputKind::human,
            "json"|"Json" => OutputKind::json,
            _ => return Err(format!("unknown output kind: {:?}", s))
        })
    }
}

impl OutputKind {
    pub const VARIANTS: &'static [&'static str] = &["human", "json"];
}

#[derive(Debug, Clap)]
#[clap(about = "Interact with crates.io from the command-line")]
#[clap(setting = clap::AppSettings::ColoredHelp)]
#[clap(setting = clap::AppSettings::ColorAuto)]
pub struct Parsed {
    #[clap(subcommand)]
    pub sub: Option<SubCommands>,
}

#[derive(Debug, Clap)]
pub enum SubCommands {
    /// show all recently changed crates
    ///
    /// The output of this command is based on the state of the current crates.io repository clone.
    /// It will remember the last result, so that the next invocation might yield different (or no)
    /// changed crates at all.
    /// Please note that the first query is likely to yield more than 40000 results!
    /// The first invocation may be slow as it might have to clone the crates.io index.
    #[clap(display_order = 1)]
    #[cfg(feature = "recent-changes")]
    RecentChanges {
        #[clap(short = 'r', long, name = "REPO")]
        /// Path to the possibly existing crates.io repository clone. If unset, it will be cloned to a temporary spot.
        repository: Option<PathBuf>,
        #[clap(long = "output", short = 'o', possible_values = &OutputKind::VARIANTS, default_value = "human")]
        /// The type of output to produce
        output_format: OutputKind,
    },
    /// search crates interactively
    #[cfg(feature = "search")]
    #[clap(display_order = 2)]
    Search,
    /// list crates by a particular criterion
    #[clap(display_order = 3)]
    #[cfg(feature = "list")]
    List {
        #[clap(subcommand)]
        cmd: ListCmd,
        #[clap(long = "output", short = 'o', possible_values = &OutputKind::VARIANTS, default_value = "human")]
        /// The type of output to produce
        output_format: OutputKind,
    },
    /// Invoke a hackable miner for crates.io
    #[cfg(feature = "mine")]
    #[clap(display_order = 4)]
    Criner(criner_cli::Args),
}

#[derive(Clap, Debug)]
pub enum ListCmd {
    /// crates for the given user id
    ByUser {
        /// The numerical id of your user, e.g. 980. Currently there is no way
        /// to easily obtain it though, so you will have to debug actual
        /// crates.io calls in your browser - the /me response contains all
        /// user data. Use any string to receive *all* crates!
        id: u32,
    },
}
