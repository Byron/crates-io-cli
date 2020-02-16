use crate::tui::utils::{block_width, draw_text_nowrap};
use crate::tui::{utils::rect, Line};
use tui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Block, Borders, Widget},
};

pub fn pane(lines: &[Line], bound: Rect, buf: &mut Buffer) {
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

    let bound = block.inner(bound);
    let mut offset = 0;
    for (line, info) in lines.windows(2).enumerate() {
        let (info, next_info) = (&info[0], &info[1]);
        let line = line + offset;
        if line >= bound.height as usize {
            break;
        }
        let line_bound = rect::line_bound(bound, line);
        match info {
            Line::Title(text) => {
                let blocks_drawn = draw_text_nowrap(line_bound, buf, text, None);
                let lines_rect = rect::offset_x(line_bound, blocks_drawn + 1);
                for x in lines_rect.left()..lines_rect.right() {
                    buf.get_mut(x, lines_rect.y).symbol = "─".into();
                }
                offset += 1;
            }
            Line::Text(text) => {
                draw_text_nowrap(rect::offset_x(line_bound, 1), buf, text, None);
            }
        };
        if let Line::Title(_) = next_info {
            offset += 1;
        }
    }
}
