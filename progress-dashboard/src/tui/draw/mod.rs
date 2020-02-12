mod tasks;

use crate::tui::draw;
use crate::{TreeKey, TreeValue};
use tui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Block, Borders, Widget},
};

pub fn all(
    title: impl AsRef<str>,
    entries: Vec<(TreeKey, TreeValue)>,
    bound: Rect,
    buf: &mut Buffer,
) -> Vec<(TreeKey, TreeValue)> {
    let mut task_progress = Block::default().title(title.as_ref()).borders(Borders::ALL);
    task_progress.draw(bound, buf);
    if bound.width < 4 || bound.height < 4 {
        return entries;
    }

    let bound = task_progress.inner(bound);
    draw::tasks::pane(entries, bound, buf)
}
