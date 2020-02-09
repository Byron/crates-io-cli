use dashmap::DashMap;
use std::sync::Arc;

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
            tree: Arc::new(DashMap::with_capacity(100)),
        }
    }

    pub fn init(&mut self, max: Option<ProgressStep>, unit: Option<&'static str>) {
        self.tree.get_mut(&self.key).map(|mut r| {
            *r.value_mut() = Some(Progress {
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
            r.value_mut().as_mut().map(|p| p.step = step);
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

type TreeId = u32; // NOTE: This means we will show weird behaviour if there are more than 2^16 tasks at the same time on a level
pub type ProgressStep = u32;

#[derive(Copy, Clone, Default, Hash, Eq, PartialEq, Ord, PartialOrd, Debug)]
struct Key((Option<TreeId>, Option<TreeId>, Option<TreeId>));

impl Key {
    fn add_child(self, child_id: TreeId) -> Key {
        Key(match self {
            Key((None, None, None)) => (Some(child_id), None, None),
            Key((a, None, None)) => (a, Some(child_id), None),
            Key((a, b, None)) => (a, b, Some(child_id)),
            Key((a, b, _c)) => {
                log::warn!("Maximum nesting level reached. Adding tasks to current parent");
                (a, b, Some(child_id))
            }
        })
    }
}

#[derive(Copy, Clone, Default, Hash, Eq, PartialEq, Ord, PartialOrd, Debug)]
struct Progress {
    step: ProgressStep,
    done_at: Option<ProgressStep>,
    unit: Option<&'static str>,
}
