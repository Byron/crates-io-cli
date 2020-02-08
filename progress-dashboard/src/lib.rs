use dashmap::DashMap;
use std::sync::Arc;

#[derive(Clone, Default, Debug)]
pub struct Config {}

impl Config {
    pub fn create(self) -> TreeRoot {
        TreeRoot::new()
    }
}

#[derive(Clone, Debug)]
pub struct TreeRoot {
    title: String,
    key: Key,
    child_count: TreeId,
    tree: Arc<DashMap<Key, Option<Progress>>>,
}

impl Drop for TreeRoot {
    fn drop(&mut self) {
        self.tree.remove(&self.key);
    }
}

impl TreeRoot {
    pub fn new() -> TreeRoot {
        TreeRoot {
            title: String::new(),
            child_count: 0,
            key: Key((None, None, None)),
            tree: Arc::new(DashMap::new()),
        }
    }

    pub fn init(&mut self, max: Option<ProgressStep>) {
        self.tree.get_mut(&self.key).map(|mut r| {
            *r.value_mut() = Some(Progress {
                step: 0,
                done_at: max,
            })
        });
    }

    pub fn add_child(&mut self, title: impl Into<String>) -> TreeRoot {
        let child_key = self.key.add_child(self.child_count);
        self.tree.insert(child_key, None);
        self.child_count = self.child_count.wrapping_add(1);
        TreeRoot {
            child_count: 0,
            title: title.into(),
            key: child_key,
            tree: self.tree.clone(),
        }
    }
}

type TreeId = u8;
pub type ProgressStep = u32;

#[derive(Copy, Clone, Default, Hash, Eq, PartialEq, Ord, PartialOrd, Debug)]
struct Key((Option<u8>, Option<u8>, Option<u8>));

impl Key {
    fn add_child(self, child_id: u8) -> Key {
        Key(match self {
            Key((None, None, None)) => (Some(child_id), None, None),
            _ => unimplemented!(),
        })
    }
}

#[derive(Copy, Clone, Default, Hash, Eq, PartialEq, Ord, PartialOrd, Debug)]
struct Progress {
    step: ProgressStep,
    done_at: Option<ProgressStep>,
}
