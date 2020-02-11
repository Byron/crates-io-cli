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
        }
    }
}

impl From<Config> for TreeRoot {
    fn from(Config { initial_capacity }: Config) -> Self {
        TreeRoot {
            inner: Arc::new(Mutex::new(Tree {
                child_id: 0,
                key: Key::default(),
                tree: Arc::new(DashMap::with_capacity(initial_capacity)),
            })),
        }
    }
}

/// The top-level of the progress tree
#[derive(Clone, Debug)]
pub struct TreeRoot {
    inner: Arc<Mutex<Tree>>,
}

impl TreeRoot {
    pub fn new() -> TreeRoot {
        Config::default().into()
    }
    pub fn add_child(&self, title: impl Into<String>) -> Tree {
        self.inner.lock().add_child(title)
    }

    pub fn sorted_snapshot(&self, out: &mut Vec<(Key, TreeValue)>) {
        out.clear();
        out.extend(
            self.inner
                .lock()
                .tree
                .iter()
                .map(|r| (r.key().clone(), r.value().clone())),
        );
        out.sort_by_key(|t| t.0);
    }
}

#[derive(Debug)]
pub struct Tree {
    key: Key,
    child_id: TreeId,
    tree: Arc<DashMap<Key, TreeValue>>,
}

impl Drop for Tree {
    fn drop(&mut self) {
        self.tree.remove(&self.key);
    }
}

impl Tree {
    pub fn init(&mut self, max: Option<ProgressStep>, unit: Option<&'static str>) {
        self.tree.get_mut(&self.key).map(|mut r| {
            r.value_mut().progress = Some(Progress {
                done_at: max,
                unit,
                ..Default::default()
            })
        });
    }

    pub fn set(&mut self, step: ProgressStep) {
        self.tree.get_mut(&self.key).map(|mut r| {
            // NOTE: since we wrap around, if there are more tasks than we can have IDs for,
            // and if all these tasks are still alive, two progress trees may see the same ID
            // when these go out of scope, they delete the key and the other tree will not find
            // its value anymore. Besides, it's probably weird to see tasks changing their progress
            // all the time…
            r.value_mut().progress.as_mut().map(|p| p.step = step);
        });
    }

    pub fn add_child(&mut self, title: impl Into<String>) -> Tree {
        let child_key = self.key.add_child(self.child_id);
        self.tree.insert(
            child_key,
            TreeValue {
                title: title.into(),
                progress: None,
            },
        );
        self.child_id = self.child_id.wrapping_add(1);
        Tree {
            child_id: 0,
            key: child_key,
            tree: self.tree.clone(),
        }
    }
}

type TreeId = u16; // NOTE: This means we will show weird behaviour if there are more than 2^16 tasks at the same time on a level
pub type ProgressStep = u32;

#[derive(Copy, Clone, Default, Hash, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub struct Key(
    (
        Option<TreeId>,
        Option<TreeId>,
        Option<TreeId>,
        Option<TreeId>,
    ),
);

impl Key {
    fn add_child(self, child_id: TreeId) -> Key {
        Key(match self {
            Key((None, None, None, None)) => (Some(child_id), None, None, None),
            Key((a, None, None, None)) => (a, Some(child_id), None, None),
            Key((a, b, None, None)) => (a, b, Some(child_id), None),
            Key((a, b, c, None)) => (a, b, c, Some(child_id)),
            Key((a, b, c, _d)) => {
                log::warn!("Maximum nesting level reached. Adding tasks to current parent");
                (a, b, c, Some(child_id))
            }
        })
    }

    pub fn level(&self) -> u8 {
        match self {
            Key((None, None, None, None)) => 0,
            Key((Some(_), None, None, None)) => 1,
            Key((Some(_), Some(_), None, None)) => 2,
            Key((Some(_), Some(_), Some(_), None)) => 3,
            Key((Some(_), Some(_), Some(_), Some(_))) => 4,
            _ => unreachable!("This is a bug - Keys follow a certain pattern"),
        }
    }

    pub const fn max_level() -> u8 {
        4
    }
}

#[derive(Copy, Clone, Default, Hash, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub struct Progress {
    pub step: ProgressStep,
    pub done_at: Option<ProgressStep>,
    pub unit: Option<&'static str>,
}

impl Progress {
    pub fn fraction(&self) -> Option<f32> {
        self.done_at
            .map(|done_at| self.step as f32 / done_at as f32)
    }
}

#[derive(Clone, Default, Hash, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub struct TreeValue {
    pub title: String,
    pub progress: Option<Progress>,
}
