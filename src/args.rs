#[cfg(feature = "mine")]
use criner_cli;

use clap::Parser;
#[cfg(any(feature = "mine", feature = "recent-changes"))]
use std::path::PathBuf;

#[derive(Debug, Clone, clap::ValueEnum)]
pub enum OutputKind {
    Human,
    Json,
}

#[derive(Debug, Parser)]
#[clap(about = "Interact with crates.io from the command-line")]
pub struct Parsed {
    #[clap(subcommand)]
    pub sub: Option<SubCommands>,
}

#[derive(Debug, Parser)]
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
        #[clap(long = "output", short = 'o', value_enum, default_value_t = OutputKind::Human)]
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
        #[clap(long = "output", short = 'o', value_enum, default_value_t = OutputKind::Human)]
        /// The type of output to produce
        output_format: OutputKind,
    },
    /// Invoke a hackable miner for crates.io
    #[cfg(feature = "mine")]
    #[clap(display_order = 4)]
    Criner(criner_cli::Args),
}

#[derive(Parser, Debug)]
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
