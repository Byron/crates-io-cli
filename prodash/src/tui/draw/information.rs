use crate::tui::Line;
use tui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Block, Borders, Widget},
};

pub fn pane(_lines: &[Line], bound: Rect, buf: &mut Buffer) {
    let mut block = Block::default()
        .title("Information")
        .borders(Borders::TOP | Borders::LEFT);
    block.draw(bound, buf);
}
