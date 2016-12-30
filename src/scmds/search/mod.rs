#[cfg(unix)]
mod structs;
#[cfg(unix)]
mod interactive;

#[cfg(unix)]
pub use self::interactive::handle_interactive_search;

#[cfg(windows)]
mod imp {
    use std;
    use clap;

    pub fn handle_interactive_search(_args: &clap::ArgMatches) {
        println!("Interactive search is not supported. Use --help to learn about alternatives.");
        std::process::exit(3);
    }
}

#[cfg(windows)]
pub use self::imp::handle_interactive_search;
