#[cfg(feature = "list")]
mod list;
#[cfg(feature = "recent-changes")]
mod recents;
#[cfg(feature = "search")]
mod search;

#[cfg(feature = "list")]
pub use self::list::{by_user, handle_list};
#[cfg(feature = "recent-changes")]
pub use self::recents::handle_recent_changes;
#[cfg(feature = "search")]
pub use self::search::handle_interactive_search;
