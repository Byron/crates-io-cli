use crate::tui::utils::{draw_text_nowrap, draw_text_nowrap_fn, intersect, GraphemeCountWriter};
use crate::{
    tree::{self},
    Progress, ProgressStep, TreeValue,
};
use std::fmt;
use tui::widgets::{Text, Widget};
use tui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    widgets::Paragraph,
};
use unicode_segmentation::UnicodeSegmentation;

struct ProgressFormat<'a>(&'a Option<Progress>, u16);

impl<'a> fmt::Display for ProgressFormat<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            Some(p) => {
                match p.done_at {
                    Some(done_at) => write!(f, "{} / {}", p.step, done_at),
                    None => write!(f, "{}", p.step),
                }?;
                if let Some(unit) = p.unit {
                    write!(f, " {}", unit)?;
                }
                Ok(())
            }
            None => write!(f, "{:─<width$}", '─', width = self.1 as usize),
        }
    }
}

pub fn draw_progress(entries: &[(tree::Key, TreeValue)], buf: &mut Buffer, bound: Rect) {
    let title_spacing = 2u16 + 1; // 2 on the left, 1 on the right
    let column_line_width = 1;
    let max_progress_label_width = entries
        .iter()
        .take(bound.height as usize)
        .map(|(_, TreeValue { progress, .. })| progress)
        .fold(0, |state, progress| match progress {
            progress @ Some(_) => {
                use std::io::Write;
                let mut w = GraphemeCountWriter::default();
                write!(w, "{}", ProgressFormat(progress, 0)).expect("never fails");
                state.max(w.0)
            }
            None => state,
        });
    let max_title_width = entries.iter().take(bound.height as usize).fold(
        0,
        |state, (key, TreeValue { progress, title })| match progress {
            None => state
                .max(title.graphemes(true).count() + key.level() as usize + title_spacing as usize),
            Some(_) => state,
        },
    );

    for (line, (key, TreeValue { progress, title })) in
        entries.iter().take(bound.height as usize).enumerate()
    {
        let progress_text = format!(
            " {progress}",
            progress = ProgressFormat(progress, bound.width.saturating_sub(title_spacing))
        );

        let y = bound.y + line as u16;
        let progress_bar_info = if let Some(fraction) = progress.and_then(|p| p.fraction()) {
            let bar_bound = intersect(
                Rect {
                    x: bound.x + column_line_width,
                    y,
                    height: 1,
                    ..bound
                },
                bound,
            );
            Some(draw_progress_bar(buf, bar_bound, fraction))
        } else {
            None
        };

        let mut progress_rect = intersect(
            Rect {
                x: bound.x,
                y,
                height: 1,
                ..bound
            },
            bound,
        );
        draw_text_nowrap(progress_rect, buf, "│", None);

        progress_rect = intersect(
            Rect {
                x: progress_rect.x + column_line_width,
                width: bound.width.saturating_sub(column_line_width),
                ..progress_rect
            },
            bound,
        );
        match progress_bar_info.map(|(bound, style)| {
            move |_t: &str, x: u16, _y: u16| {
                if x < bound.right() {
                    style
                } else {
                    Style::default()
                }
            }
        }) {
            Some(style_fn) => {
                draw_text_nowrap_fn(progress_rect, buf, progress_text, style_fn);
            }
            None => {
                draw_text_nowrap(progress_rect, buf, progress_text, None);
                // we have progress, but no upper limit
                if let Some((step, None)) = progress.as_ref().map(|p| (p.step, p.done_at.as_ref()))
                {
                    let bar_rect = intersect(
                        Rect {
                            x: bound.x + max_progress_label_width as u16,
                            y,
                            height: 1,
                            ..bound
                        },
                        bound,
                    );
                    draw_spinner(buf, bar_rect, step, line);
                }
            }
        }

        if progress.is_none() {
            let center_rect = intersect(
                Rect {
                    x: bound.x
                        + column_line_width
                        + (bound.width.saturating_sub(max_title_width as u16)) / 2,
                    y,
                    width: max_title_width as u16,
                    height: 1,
                },
                bound,
            );
            let title_text = format!(
                " {:‧<prefix_count$} {} ",
                "",
                title,
                prefix_count = key.level() as usize
            );
            draw_text_nowrap(center_rect, buf, title_text, None);
        }
    }
}

fn draw_spinner(buf: &mut Buffer, bound: Rect, step: ProgressStep, seed: usize) {
    if bound.width == 0 {
        return;
    }
    let step = step as usize;
    let x = bound.x + ((step + seed) % bound.width as usize) as u16;
    let width = 5;
    let bound = intersect(Rect { x, width, ..bound }, bound);
    tui_react::fill_background(bound, buf, Color::White);
}

fn draw_progress_bar(buf: &mut Buffer, bound: Rect, fraction: f32) -> (Rect, Style) {
    if bound.width == 0 {
        return (Rect::default(), Style::default());
    }
    let fractional_progress_rect = Rect {
        width: ((bound.width as f32 * fraction).ceil() as u16).min(bound.width),
        ..bound
    };
    let color = if fraction >= 0.8 {
        Color::Green
    } else {
        Color::Yellow
    };
    tui_react::fill_background(fractional_progress_rect, buf, color);
    (
        fractional_progress_rect,
        Style::default().bg(color).fg(Color::Black),
    )
}

pub fn draw_tree_prefix(entries: &[(tree::Key, TreeValue)], buf: &mut Buffer, bound: Rect) -> u16 {
    let mut max_prefix_len = 0;
    for (line, (key, TreeValue { progress, title })) in
        entries.iter().take(bound.height as usize).enumerate()
    {
        let tree_prefix = format!(
            "{:>width$} {} ",
            if key.level() == 1 {
                "‧"
            } else {
                if progress.is_none() {
                    "…"
                } else {
                    "└"
                }
            },
            if progress.is_none() { "" } else { &title },
            width = key.level() as usize
        );
        let line_rect = intersect(
            Rect {
                y: bound.y + line as u16,
                height: 1,
                ..bound
            },
            bound,
        );
        max_prefix_len = max_prefix_len.max(draw_text_nowrap(line_rect, buf, tree_prefix, None));
    }
    max_prefix_len
}

pub fn draw_overflow<'a>(
    entries: impl Iterator<Item = &'a (tree::Key, TreeValue)>,
    buf: &mut Buffer,
    bound: Rect,
) {
    let (count, mut progress_percent) = entries.fold(
        (0usize, 0f32),
        |(count, progress_percent), (_key, value)| {
            let progress = value
                .progress
                .and_then(|p| p.fraction().map(|f| f * 100.0))
                .unwrap_or_default();
            (count + 1, progress_percent + progress)
        },
    );
    progress_percent /= count as f32;
    Paragraph::new(
        [Text::Raw(
            format!("…and {} more -- {:4.01}%", count, progress_percent).into(),
        )]
        .iter(),
    )
    .draw(bound, buf);
}
