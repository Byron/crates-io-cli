mod information;
mod messages;
mod tasks;

use crate::{
    tui::draw, tui::utils::block_width, tui::utils::rect, tui::Line, tui::State, Message, TreeKey,
    TreeValue,
};
use tui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Block, Borders, Widget},
};

pub fn all(
    state: &mut State,
    entries: &[(TreeKey, TreeValue)],
    messages: &[Message],
    bound: Rect,
    buf: &mut Buffer,
) {
    let mut window = Block::default().title(&state.title).borders(Borders::ALL);
    window.draw(bound, buf);
    if bound.width < 4 || bound.height < 4 {
        return;
    }

    let border_width = 1;
    draw::tasks::headline(
        &entries,
        state.duration_per_frame,
        buf,
        rect::offset_x(
            Rect {
                height: 1,
                width: bound.width.saturating_sub(border_width),
                ..bound
            },
            block_width(&state.title) + (border_width * 2),
        ),
    );

    let inner = window.inner(bound);
    let (tasks_pane, messages_pane, info_pane) = compute_pane_bounds(
        if state.hide_messages { &[] } else { messages },
        inner,
        state.messages_fullscreen,
        if state.hide_info {
            &[]
        } else {
            &state.information
        },
    );

    draw::tasks::pane(&entries, tasks_pane, &mut state.task_offset, buf);
    if let Some(messages_pane) = messages_pane {
        draw::messages::pane(
            messages,
            messages_pane,
            rect::line_bound(bound, bound.height.saturating_sub(1) as usize),
            &mut state.message_offset,
            buf,
        );
    }

    if let Some(info_pane) = info_pane {
        draw::information::pane(&state.information, info_pane, buf);
    }
}

fn compute_pane_bounds(
    messages: &[Message],
    inner: Rect,
    messages_fullscreen: bool,
    info: &[Line],
) -> (Rect, Option<Rect>, Option<Rect>) {
    if messages.is_empty() {
        (inner, None, compute_info_bound(inner, info))
    } else {
        let (task_percent, messages_percent) = if messages_fullscreen {
            (0.1, 0.9)
        } else {
            (0.75, 0.25)
        };
        let tasks_height: u16 = (inner.height as f32 * task_percent).ceil() as u16;
        let messages_height: u16 = (inner.height as f32 * messages_percent).floor() as u16;
        if messages_height < 2 {
            (inner, None, compute_info_bound(inner, info))
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
                compute_info_bound(inner, info),
            )
        }
    }
}

fn compute_info_bound(bound: Rect, info: &[Line]) -> Option<Rect> {
    if info.is_empty() {
        return None;
    }
    let max_line_width = info.iter().fold(0, |state, l| {
        state.max(block_width(match l {
            Line::Text(s) | Line::Title(s) => s,
        }))
    });
    Some(rect::snap_to_right(
        bound,
        (bound.width / 2).min(max_line_width),
    ))
}
