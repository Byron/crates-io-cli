mod tasks;

use crate::{tui::draw, tui::utils::rect, Message, TreeKey, TreeValue};
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
    entries: &[(TreeKey, TreeValue)],
    messages: &[Message],
    bound: Rect,
    buf: &mut Buffer,
) {
    let mut window = Block::default().title(title.as_ref()).borders(Borders::ALL);
    window.draw(bound, buf);
    if bound.width < 4 || bound.height < 4 {
        return;
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

    let inner = window.inner(bound);
    let (tasks_pane, messages_pane) = compute_pane_bounds(messages, inner);

    draw::tasks::pane(&entries, tasks_pane, buf);
    if let Some(messages_pane) = messages_pane {
        draw::messages::pane(messages, messages_pane, buf);
    }
}

fn compute_pane_bounds(messages: &[Message], inner: Rect) -> (Rect, Option<Rect>) {
    if messages.is_empty() {
        (inner, None::<Rect>)
    } else {
        let tasks_height: u16 = (inner.height as f32 * 0.75).ceil() as u16;
        let messages_height: u16 = (inner.height as f32 * 0.25).floor() as u16;
        if messages_height < 2 {
            (inner, None)
        } else {
            let messages_title = 1u16;
            let new_messages_height =
                messages_height.min((messages.len() + messages_title as usize) as u16);
            let tasks_height = tasks_height.saturating_add(messages_height - new_messages_height);
            let messages_height = new_messages_height;
            (
                Rect {
                    height: tasks_height,
                    ..inner
                },
                Some(rect::intersect(
                    Rect {
                        y: tasks_height + messages_title,
                        height: messages_height,
                        ..inner
                    },
                    inner,
                )),
            )
        }
    }
}

mod messages {
    use crate::tui::utils::{draw_text_nowrap, rect};
    use crate::Message;
    use tui::{
        buffer::Buffer,
        layout::Rect,
        widgets::{Block, Borders, Widget},
    };

    pub fn pane(messages: &[Message], bound: Rect, buf: &mut Buffer) {
        let mut block = Block::default().title("Messages").borders(Borders::TOP);
        block.draw(bound, buf);

        let bound = block.inner(bound);
        for (line, Message { message, .. }) in
            messages.iter().take(bound.height as usize).enumerate()
        {
            let line_bound = rect::line_bound(bound, line);
            draw_text_nowrap(line_bound, buf, message, None);
        }
    }
}
