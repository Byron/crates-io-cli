use crate::Config;
use dashmap::DashMap;
use parking_lot::Mutex;
use std::{sync::Arc, time::SystemTime};

/// The top-level of the progress tree
#[derive(Clone, Debug)]
pub struct TreeRoot {
    pub(crate) inner: Arc<Mutex<Tree>>,
}

impl TreeRoot {
    pub fn new() -> TreeRoot {
        Config::default().into()
    }
    pub fn messages_capacity(&self) -> usize {
        self.inner.lock().messages.lock().buf.capacity()
    }
    pub fn num_tasks(&self) -> usize {
        self.inner.lock().tree.len()
    }
    pub fn add_child(&self, title: impl Into<String>) -> Tree {
        self.inner.lock().add_child(title)
    }

    pub fn sorted_snapshot(&self, out: &mut Vec<(TreeKey, TreeValue)>) {
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

    pub fn copy_messages(&self, out: &mut Vec<Message>) {
        self.inner.lock().messages.lock().copy_into(out);
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum MessageLevel {
    Info,
    Failure,
    Success,
}

#[derive(Debug, Clone)]
pub struct Message {
    pub time: SystemTime,
    pub level: MessageLevel,
    pub origin: String,
    pub message: String,
}

#[derive(Debug)]
pub(crate) struct MessageRingBuffer {
    buf: Vec<Message>,
    cursor: usize,
}

impl MessageRingBuffer {
    pub fn with_capacity(capacity: usize) -> MessageRingBuffer {
        MessageRingBuffer {
            buf: Vec::with_capacity(capacity),
            cursor: 0,
        }
    }

    fn has_capacity(&self) -> bool {
        self.buf.len() < self.buf.capacity()
    }

    pub fn push_overwrite(&mut self, level: MessageLevel, origin: String, message: &str) {
        let msg = Message {
            time: SystemTime::now(),
            level,
            origin,
            message: message.to_string(),
        };
        if self.has_capacity() {
            self.buf.push(msg)
        } else {
            self.buf[self.cursor] = msg;
            self.cursor = (self.cursor + 1) % self.buf.len();
        }
    }

    pub fn copy_into(&self, out: &mut Vec<Message>) {
        out.clear();
        if self.has_capacity() {
            out.extend_from_slice(self.buf.as_slice());
        } else {
            out.extend_from_slice(&self.buf[(self.cursor + 1) % self.buf.len()..]);
            if self.cursor + 1 != self.buf.len() {
                out.extend_from_slice(&self.buf[..self.cursor]);
            }
        }
    }
}

#[derive(Debug)]
pub struct Tree {
    pub(crate) key: TreeKey,
    pub(crate) highest_child_id: TreeId,
    pub(crate) tree: Arc<DashMap<TreeKey, TreeValue>>,
    pub(crate) messages: Arc<Mutex<MessageRingBuffer>>,
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

    fn alter_progress(&mut self, f: impl FnMut(&mut Progress)) {
        self.tree.get_mut(&self.key).map(|mut r| {
            // NOTE: since we wrap around, if there are more tasks than we can have IDs for,
            // and if all these tasks are still alive, two progress trees may see the same ID
            // when these go out of scope, they delete the key and the other tree will not find
            // its value anymore. Besides, it's probably weird to see tasks changing their progress
            // all the time…
            r.value_mut().progress.as_mut().map(f);
        });
    }

    pub fn set(&mut self, step: ProgressStep) {
        self.alter_progress(|p| {
            p.step = step;
            p.state = TaskState::Running;
        });
    }

    pub fn blocked(&mut self, eta: Option<SystemTime>) {
        self.alter_progress(|p| p.state = TaskState::Blocked(eta));
    }

    pub fn add_child(&mut self, title: impl Into<String>) -> Tree {
        let child_key = self.key.add_child(self.highest_child_id);
        self.tree.insert(
            child_key,
            TreeValue {
                title: title.into(),
                progress: None,
            },
        );
        self.highest_child_id = self.highest_child_id.wrapping_add(1);
        Tree {
            highest_child_id: 0,
            key: child_key,
            tree: self.tree.clone(),
            messages: self.messages.clone(),
        }
    }

    pub fn message(&mut self, level: MessageLevel, message: impl AsRef<str>) {
        self.messages.lock().push_overwrite(
            level,
            self.tree
                .get(&self.key)
                .map(|v| v.title.to_owned())
                .unwrap_or_default(),
            message.as_ref(),
        )
    }

    pub fn done(&mut self, message: impl AsRef<str>) {
        self.message(MessageLevel::Success, message)
    }
    pub fn fail(&mut self, message: impl AsRef<str>) {
        self.message(MessageLevel::Failure, message)
    }
    pub fn info(&mut self, message: impl AsRef<str>) {
        self.message(MessageLevel::Info, message)
    }
}

type TreeId = u16; // NOTE: This means we will show weird behaviour if there are more than 2^16 tasks at the same time on a level
pub type ProgressStep = u32;

#[derive(Copy, Clone, Default, Hash, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub struct TreeKey(
    (
        Option<TreeId>,
        Option<TreeId>,
        Option<TreeId>,
        Option<TreeId>,
    ),
);

impl TreeKey {
    fn add_child(self, child_id: TreeId) -> TreeKey {
        TreeKey(match self {
            TreeKey((None, None, None, None)) => (Some(child_id), None, None, None),
            TreeKey((a, None, None, None)) => (a, Some(child_id), None, None),
            TreeKey((a, b, None, None)) => (a, b, Some(child_id), None),
            TreeKey((a, b, c, None)) => (a, b, c, Some(child_id)),
            TreeKey((a, b, c, _d)) => {
                log::warn!("Maximum nesting level reached. Adding tasks to current parent");
                (a, b, c, Some(child_id))
            }
        })
    }

    pub fn level(&self) -> u8 {
        match self {
            TreeKey((None, None, None, None)) => 0,
            TreeKey((Some(_), None, None, None)) => 1,
            TreeKey((Some(_), Some(_), None, None)) => 2,
            TreeKey((Some(_), Some(_), Some(_), None)) => 3,
            TreeKey((Some(_), Some(_), Some(_), Some(_))) => 4,
            _ => unreachable!("This is a bug - Keys follow a certain pattern"),
        }
    }

    pub const fn max_level() -> u8 {
        4
    }
}

#[derive(Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub enum TaskState {
    /// Indicates a task is blocked and cannot make progress, optionally until the
    /// given time.
    Blocked(Option<SystemTime>),
    /// The task is running
    Running,
}

impl Default for TaskState {
    fn default() -> Self {
        TaskState::Running
    }
}

#[derive(Copy, Clone, Default, Hash, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub struct Progress {
    pub step: ProgressStep,
    pub done_at: Option<ProgressStep>,
    pub unit: Option<&'static str>,
    pub state: TaskState,
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
