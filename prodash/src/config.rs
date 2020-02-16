use crate::{
    tree::{Item, Key, MessageRingBuffer},
    Tree,
};
use dashmap::DashMap;
use parking_lot::Mutex;
use std::sync::Arc;

/// A way to configure new [`tree::Root`](./tree/struct.Root.html) instances
/// ```rust
/// use prodash::{Tree, TreeConfig};
/// let tree = TreeConfig::default().create();
/// let tree2 = TreeConfig { message_buffer_capacity: 100, ..TreeConfig::default() }.create();
/// ```
#[derive(Clone, Debug)]
pub struct TreeConfig {
    /// The amount of items the tree can hold without being forced to allocate
    pub initial_capacity: usize,
    /// The amount of messages we can hold before we start overwriting old ones
    pub message_buffer_capacity: usize,
}

impl TreeConfig {
    /// Create a new [`Root`](./tree/struct.Root.html) instance from the
    /// configuration within.
    pub fn create(self) -> Tree {
        self.into()
    }
}

impl Default for TreeConfig {
    fn default() -> Self {
        TreeConfig {
            initial_capacity: 100,
            message_buffer_capacity: 20,
        }
    }
}

impl From<TreeConfig> for Tree {
    fn from(
        TreeConfig {
            initial_capacity,
            message_buffer_capacity,
        }: TreeConfig,
    ) -> Self {
        Tree {
            inner: Arc::new(Mutex::new(Item {
                highest_child_id: 0,
                key: Key::default(),
                tree: Arc::new(DashMap::with_capacity(initial_capacity)),
                messages: Arc::new(Mutex::new(MessageRingBuffer::with_capacity(
                    message_buffer_capacity,
                ))),
            })),
        }
    }
}
