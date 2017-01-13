mod recents;
mod search;

pub use self::recents::{OutputKind, handle_recent_changes, Error as RecentChangesError};
pub use self::search::{handle_interactive_search, Error as SearchError};
