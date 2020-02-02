use crate::structs::OutputKind;
use std::path::PathBuf;
use structopt::StructOpt;

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
    #[structopt(display_order = 1)]
    RecentChanges {
        #[structopt(short = "r", long, name = "REPO")]
        /// Path to the possibly existing crates.io repository clone.
        repository: Option<PathBuf>,
        #[structopt(long, short = "o", possible_values = &OutputKind::variants())]
        /// The type of output to produce
        output_format: Option<OutputKind>,
    },
    /// search crates interactively
    #[structopt(display_order = 2)]
    Search,
    /// list crates by a particular criterion
    #[structopt(display_order = 3)]
    List {
        #[structopt(subcommand)]
        cmd: ListCmd,
        #[structopt(long, short = "o", possible_values = &OutputKind::variants())]
        /// The type of output to produce
        output_format: Option<OutputKind>,
    },
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
