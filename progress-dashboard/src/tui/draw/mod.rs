mod tasks;

use crate::tui::draw;
use crate::{tree, TreeValue};
use tui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Block, Borders, Widget},
};

pub fn all(
    title: impl AsRef<str>,
    entries: Vec<(tree::Key, TreeValue)>,
    bound: Rect,
    buf: &mut Buffer,
) -> Vec<(tree::Key, TreeValue)> {
    let mut task_progress = Block::default().title(title.as_ref()).borders(Borders::ALL);
    task_progress.draw(bound, buf);

    let bound = task_progress.inner(bound);
    draw::tasks::pane(entries, bound, buf)
}
