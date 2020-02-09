use dashmap::DashMap;
use parking_lot::Mutex;
use std::sync::Arc;

/// The top-level of the progress tree
#[derive(Clone, Debug)]
pub struct TreeRoot {
    inner: Arc<Mutex<Tree>>,
}

impl TreeRoot {
    pub fn new() -> TreeRoot {
        TreeRoot {
            inner: Arc::new(Mutex::new(Tree {
                title: String::new(),
                child_id: 0,
                key: Key::default(),
                tree: Arc::new(DashMap::with_capacity(100)),
            })),
        }
    }
    pub fn add_child(&self, title: impl Into<String>) -> Tree {
        self.inner.lock().add_child(title)
    }

    pub fn sorted_snapshot(&self, _out: &mut Vec<(Key, Progress)>) {
        unimplemented!()
    }
}

#[derive(Debug)]
pub struct Tree {
    title: String,
    key: Key,
    child_id: TreeId,
    tree: Arc<DashMap<Key, Option<Progress>>>,
}

impl Drop for Tree {
    fn drop(&mut self) {
        self.tree.remove(&self.key);
    }
}

impl Tree {
    pub fn init(&mut self, max: Option<ProgressStep>, unit: Option<&'static str>) {
        self.tree.get_mut(&self.key).map(|mut r| {
            *r.value_mut() = Some(Progress {
                done_at: max,
                unit,
                ..Default::default()
            })
        });
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn set(&mut self, step: ProgressStep) {
        self.tree.get_mut(&self.key).map(|mut r| {
            // NOTE: since we wrap around, if there are more tasks than we can have IDs for,
            // and if all these tasks are still alive, two progress trees may see the same ID
            // when these go out of scope, they delete the key and the other tree will not find
            // its value anymore. Besides, it's probably weird to see tasks changing their progress
            // all the time…
            r.value_mut().as_mut().map(|p| p.step = step);
        });
    }

    pub fn add_child(&mut self, title: impl Into<String>) -> Tree {
        let child_key = self.key.add_child(self.child_id);
        self.tree.insert(child_key, None);
        self.child_id = self.child_id.wrapping_add(1);
        Tree {
            child_id: 0,
            title: title.into(),
            key: child_key,
            tree: self.tree.clone(),
        }
    }
}

type TreeId = u32; // NOTE: This means we will show weird behaviour if there are more than 2^16 tasks at the same time on a level
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
}

#[derive(Copy, Clone, Default, Hash, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub struct Progress {
    step: ProgressStep,
    done_at: Option<ProgressStep>,
    unit: Option<&'static str>,
}
