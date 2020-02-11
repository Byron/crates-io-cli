use crate::tui::tasks;
use crate::tui::utils::intersect;
use crate::{tree, TreeValue};
use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::widgets::{Block, Borders, Widget};

pub fn all(
    title: impl AsRef<str>,
    entries: Vec<(tree::Key, TreeValue)>,
    bound: Rect,
    buf: &mut Buffer,
) -> Vec<(tree::Key, TreeValue)> {
    let mut progress_pane = Block::default().title(title.as_ref()).borders(Borders::ALL);
    progress_pane.draw(bound, buf);
    let mut bound = progress_pane.inner(bound);
    let is_overflowing = if entries.len() > bound.height as usize {
        bound.height = bound.height.saturating_sub(1);
        true
    } else {
        false
    };

    if !entries.is_empty() {
        let column_width = bound.width / 2;
        let max_prefix_width = {
            let prefix_area = Rect {
                width: column_width,
                ..bound
            };
            tasks::draw_tree_prefix(&entries, buf, prefix_area)
        };

        {
            let max_prefix_width = max_prefix_width;
            let progress_area = intersect(
                Rect {
                    x: bound.x + max_prefix_width,
                    ..bound
                },
                bound,
            );
            tasks::draw_progress(&entries, buf, progress_area);
        }

        if is_overflowing {
            let overflow_rect = Rect {
                y: bound.height + 1,
                height: 1,
                ..bound
            };
            tasks::draw_overflow(
                entries.iter().skip(bound.height as usize),
                buf,
                overflow_rect,
            );
        }
    }
    entries
}
