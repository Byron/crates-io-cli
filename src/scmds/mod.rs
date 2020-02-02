#[cfg(feature = "list")]
mod list;
#[cfg(feature = "recent-changes")]
mod recents;
//mod search;

#[cfg(feature = "list")]
pub use self::list::{by_user, handle_list, Error as ListError};
#[cfg(feature = "recent-changes")]
pub use self::recents::{handle_recent_changes, Error as RecentChangesError};
//pub use self::search::{handle_interactive_search, Error as SearchError};
