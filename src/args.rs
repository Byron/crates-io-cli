#[cfg(feature = "mine")]
use criner_cli;

#[cfg(any(feature = "mine", feature = "recent-changes"))]
use std::path::PathBuf;
use structopt::StructOpt;

arg_enum! {
    #[allow(non_camel_case_types)]
    #[derive(Debug)]
    pub enum OutputKind {
        human,
        json

    }
}

#[derive(Debug, StructOpt)]
#[structopt(about = "Interact with crates.io from the command-line")]
#[structopt(settings = &[clap::AppSettings::ColoredHelp, clap::AppSettings::ColorAuto])]
pub struct Parsed {
    #[structopt(subcommand)]
    pub sub: Option<SubCommands>,
}

#[derive(Debug, StructOpt)]
pub enum SubCommands {
    /// show all recently changed crates
    ///
    /// The output of this command is based on the state of the current crates.io repository clone.
    /// It will remember the last result, so that the next invocation might yield different (or no)
    /// changed crates at all.
    /// Please note that the first query is likely to yield more than 40000 results!
    /// The first invocation may be slow as it might have to clone the crates.io index.
    #[structopt(display_order = 1)]
    #[cfg(feature = "recent-changes")]
    RecentChanges {
        #[structopt(short = "r", long, name = "REPO")]
        /// Path to the possibly existing crates.io repository clone. If unset, it will be cloned to a temporary spot.
        repository: Option<PathBuf>,
        #[structopt(long = "output", short = "o", possible_values = &OutputKind::variants(), default_value = "human")]
        /// The type of output to produce
        output_format: OutputKind,
    },
    /// search crates interactively
    #[cfg(feature = "search")]
    #[structopt(display_order = 2)]
    Search,
    /// list crates by a particular criterion
    #[structopt(display_order = 3)]
    #[cfg(feature = "list")]
    List {
        #[structopt(subcommand)]
        cmd: ListCmd,
        #[structopt(long = "output", short = "o", possible_values = &OutputKind::variants(), default_value = "human")]
        /// The type of output to produce
        output_format: OutputKind,
    },
    /// Invoke a hackable miner for crates.io
    #[cfg(feature = "mine")]
    #[structopt(display_order = 4)]
    // #[structopt(flatten)]
    Criner(criner_cli::SubCommands),
}

#[derive(StructOpt, Debug)]
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
