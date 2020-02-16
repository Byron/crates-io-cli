use crate::tui::utils::{block_width, draw_text_nowrap};
use crate::tui::{utils::rect, Line};
use tui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Block, Borders, Widget},
};

pub fn pane(_lines: &[Line], bound: Rect, buf: &mut Buffer) {
    let mut block = Block::default()
        .title("Information")
        .borders(Borders::TOP | Borders::BOTTOM);
    block.draw(bound, buf);

    let help_text = " ⨯ = [ | ▢ = { ";
    draw_text_nowrap(
        rect::snap_to_right(bound, block_width(help_text)),
        buf,
        help_text,
        None,
    );
}
