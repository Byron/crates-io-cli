#[cfg(unix)]
mod structs;
#[cfg(unix)]
mod interactive;

#[cfg(unix)]
pub use self::interactive::handle_interactive_search;

#[cfg(windows)]
pub fn handle_interactive_search(_args: &clap::ArgMatches) {
    use std;
    println!("Interactive search is not supported. Use --help to learn about alternatives.");
    std::process::exit(3);
}

