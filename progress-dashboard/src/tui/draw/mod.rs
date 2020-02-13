mod tasks;

use crate::tui::utils::rect;
use crate::{tui::draw, TreeKey, TreeValue};
use tui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Block, Borders, Widget},
};
use unicode_segmentation::UnicodeSegmentation;

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

    draw::tasks::headline(
        &entries,
        buf,
        rect::offset_x(
            Rect { height: 1, ..bound },
            (title.as_ref().graphemes(true).count() + 2) as u16,
        ),
    );

    let bound = task_progress.inner(bound);
    draw::tasks::pane(entries, bound, buf)
}
