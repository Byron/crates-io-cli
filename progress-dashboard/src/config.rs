use crate::{MessageRingBuffer, Tree, TreeKey, TreeRoot};
use dashmap::DashMap;
use parking_lot::Mutex;
use std::sync::Arc;

/// A way to configure new `TreeRoot` instances
/// ```rust
/// use progress_dashboard::{TreeRoot, Config};
/// let tree: TreeRoot = Config::default().create();
/// ```
#[derive(Clone, Debug)]
pub struct Config {
    /// The amount of items the tree can hold without being forced to allocate
    pub initial_capacity: usize,
    /// The amount of messages we can hold before we start overwriting old ones
    pub message_buffer_capacity: usize,
}

impl Config {
    pub fn create(self) -> TreeRoot {
        self.into()
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            initial_capacity: 100,
            message_buffer_capacity: 100,
        }
    }
}

impl From<Config> for TreeRoot {
    fn from(
        Config {
            initial_capacity,
            message_buffer_capacity,
        }: Config,
    ) -> Self {
        TreeRoot {
            inner: Arc::new(Mutex::new(Tree {
                highest_child_id: 0,
                key: TreeKey::default(),
                tree: Arc::new(DashMap::with_capacity(initial_capacity)),
                messages: Arc::new(Mutex::new(MessageRingBuffer::with_capacity(
                    message_buffer_capacity,
                ))),
            })),
        }
    }
}
