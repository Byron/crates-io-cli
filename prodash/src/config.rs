use crate::{MessageRingBuffer, Tree, TreeKey, TreeRoot};
use dashmap::DashMap;
use parking_lot::Mutex;
use std::sync::Arc;

/// A way to configure new [`TreeRoot`](./struct.TreeRoot.html) instances
/// ```rust
/// use prodash::{TreeRoot, Config};
/// let tree = Config::default().create();
/// let tree2 = Config { message_buffer_capacity: 100, ..Config::default() }.create();
/// ```
#[derive(Clone, Debug)]
pub struct Config {
    /// The amount of items the tree can hold without being forced to allocate
    pub initial_capacity: usize,
    /// The amount of messages we can hold before we start overwriting old ones
    pub message_buffer_capacity: usize,
}

impl Config {
    /// Create a new [`TreeRoot`](./struct.TreeRoot.html) instance from the
    /// configuration within.
    pub fn create(self) -> TreeRoot {
        self.into()
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            initial_capacity: 100,
            message_buffer_capacity: 20,
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
