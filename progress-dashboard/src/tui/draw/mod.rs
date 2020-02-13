mod tasks;

use crate::{tui::draw, tui::utils::rect, TreeKey, TreeValue};
use std::time::Duration;
use tui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Block, Borders, Widget},
};
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

pub fn all(
    title: impl AsRef<str>,
    duration_per_frame: Duration,
    entries: Vec<(TreeKey, TreeValue)>,
    bound: Rect,
    buf: &mut Buffer,
) -> Vec<(TreeKey, TreeValue)> {
    let mut task_progress = Block::default().title(title.as_ref()).borders(Borders::ALL);
    task_progress.draw(bound, buf);
    if bound.width < 4 || bound.height < 4 {
        return entries;
    }

    let border_width = 1;
    draw::tasks::headline(
        &entries,
        duration_per_frame,
        buf,
        rect::offset_x(
            Rect {
                height: 1,
                width: bound.width.saturating_sub(border_width),
                ..bound
            },
            (title
                .as_ref()
                .graphemes(true)
                .map(|s| s.width())
                .sum::<usize>()
                + 2) as u16,
        ),
    );

    let bound = task_progress.inner(bound);
    draw::tasks::pane(entries, bound, buf)
}
