use crate::{
    tui::utils::{draw_text_nowrap, rect},
    Message, MessageLevel,
};
use std::time::SystemTime;
use tui::style::{Color, Modifier, Style};
use tui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Block, Borders, Widget},
};
use unicode_width::UnicodeWidthStr;

const TIME_COLUMN_PREFIX: u16 = "20-02-13T".len() as u16;

const TIME_COLUMN_SUFFIX: u16 = "00:51:45".len() as u16;

pub fn pane(messages: &[Message], bound: Rect, buf: &mut Buffer) {
    let mut block = Block::default().title("Messages").borders(Borders::TOP);
    block.draw(bound, buf);

    let bound = block.inner(bound);
    for (
        line,
        Message {
            time,
            message,
            level,
            ..
        },
    ) in messages
        .iter()
        .take(bound.height as usize)
        .rev()
        .enumerate()
    {
        let line_bound = rect::line_bound(bound, line);
        let (time_bound, level_bound, message_bound) = compute_bounds(line_bound);
        if let Some(time_bound) = time_bound {
            draw_text_nowrap(time_bound, buf, format_time_column(time), None);
        }
        if let Some(level_bound) = level_bound {
            draw_text_nowrap(
                level_bound,
                buf,
                format_level_column(*level),
                Some(level_to_style(*level)),
            );
            draw_text_nowrap(
                rect::offset_x(level_bound, LEVEL_TEXT_WIDTH),
                buf,
                rect::VERTICAL_LINE,
                None,
            );
        }
        draw_text_nowrap(message_bound, buf, message, None);
    }
}

const LEVEL_TEXT_WIDTH: u16 = 4;
fn format_level_column(level: MessageLevel) -> &'static str {
    use MessageLevel::*;
    match level {
        Info => "info",
        Failure => "fail",
        Success => "done",
    }
}

fn level_to_style(level: MessageLevel) -> Style {
    use MessageLevel::*;
    Style::default()
        .fg(Color::Black)
        .modifier(Modifier::BOLD)
        .bg(match level {
            Info => Color::White,
            Failure => Color::Red,
            Success => Color::Green,
        })
}

fn format_time_column(time: &SystemTime) -> String {
    format!(
        "{}{}",
        String::from_utf8_lossy(
            &format!("{}", humantime::format_rfc3339_seconds(*time)).as_bytes()[(TIME_COLUMN_PREFIX
                + 2)
                as usize
                ..(TIME_COLUMN_PREFIX + TIME_COLUMN_SUFFIX + 2) as usize],
        ),
        rect::VERTICAL_LINE
    )
}

fn compute_bounds(line: Rect) -> (Option<Rect>, Option<Rect>, Rect) {
    let vertical_line_width = rect::VERTICAL_LINE.width() as u16;
    let mythical_offset_we_should_not_need = 1;

    let time_bound = Rect {
        width: TIME_COLUMN_SUFFIX + vertical_line_width,
        ..line
    };

    let mut cursor = time_bound.width + mythical_offset_we_should_not_need;
    let level_bound = Rect {
        x: cursor,
        width: LEVEL_TEXT_WIDTH + vertical_line_width,
        ..line
    };
    cursor += level_bound.width;

    let message_bound = rect::intersect(rect::offset_x(line, cursor), line);
    if message_bound.width < 30 {
        return (None, None, line);
    }
    (Some(time_bound), Some(level_bound), message_bound)
}
