#[cfg(feature = "recent-changes")]
mod recents;
//mod list;
//mod search;

#[cfg(feature = "recent-changes")]
pub use self::recents::{handle_recent_changes, Error as RecentChangesError};
//pub use self::list::{by_user, handle_list, Error as ListError};
//pub use self::search::{handle_interactive_search, Error as SearchError};
