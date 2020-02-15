use crate::{
    tui::utils::{draw_text_nowrap, rect},
    Message,
};
use std::time::SystemTime;
use tui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Block, Borders, Widget},
};

const TIME_COLUMN_WIDTH: u16 = "2020-02-13T00:51:45".len() as u16;

pub fn pane(messages: &[Message], bound: Rect, buf: &mut Buffer) {
    let mut block = Block::default().title("Messages").borders(Borders::TOP);
    block.draw(bound, buf);

    let bound = block.inner(bound);
    for (line, Message { time, message, .. }) in messages
        .iter()
        .take(bound.height as usize)
        .rev()
        .enumerate()
    {
        let line_bound = rect::line_bound(bound, line);
        let (time_bound, message_bound) = compute_bounds(line_bound);
        if let Some(time_bound) = time_bound {
            draw_text_nowrap(time_bound, buf, format_time_column(time), None);
        }
        draw_text_nowrap(message_bound, buf, message, None);
    }
}

fn format_time_column(time: &SystemTime) -> String {
    format!(
        "{}{}",
        String::from_utf8_lossy(
            &format!("{}", humantime::format_rfc3339(*time),).as_bytes()
                [..TIME_COLUMN_WIDTH as usize],
        ),
        rect::VERTICAL_LINE
    )
}

fn compute_bounds(line: Rect) -> (Option<Rect>, Rect) {
    let vertical_line_width = 1u16;
    let time_bound = Rect {
        width: TIME_COLUMN_WIDTH + vertical_line_width,
        ..line
    };
    let message_bound = rect::intersect(rect::offset_x(line, time_bound.width), line);
    if message_bound.width < 30 {
        return (None, line);
    }
    (Some(time_bound), message_bound)
}
