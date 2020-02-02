#[cfg(unix)]
mod interactive;
#[cfg(unix)]
mod structs;

mod error;
pub use self::error::Error;

#[cfg(unix)]
pub use self::interactive::handle_interactive_search;

#[cfg(windows)]
pub fn handle_interactive_search() -> Result<(), Error> {
    println!("Interactive search is not supported. Use --help to learn about alternatives.");
    std::process::exit(3);
}
