use crate::{
    tree::{self, TreeRoot},
    Progress, ProgressStep, TreeValue,
};
use futures_timer::Delay;

use futures::{channel::mpsc, future::select, future::Either, io::Error, SinkExt, StreamExt};
use std::{fmt, io, time::Duration};
use termion::{event::Key, input::TermRead, raw::IntoRawMode, screen::AlternateScreen};
use tui::style::{Color, Style};
use tui::{
    backend::TermionBackend,
    buffer::Buffer,
    layout::Rect,
    widgets::{Block, Borders, Widget},
    widgets::{Paragraph, Text},
};
use tui_react::Terminal;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Clone)]
pub struct Config {
    pub frames_per_second: u8,
}

pub fn render(
    progress: TreeRoot,
    Config { frames_per_second }: Config,
) -> Result<impl std::future::Future<Output = ()>, std::io::Error> {
    let mut terminal = {
        let stdout = io::stdout().into_raw_mode()?;
        let stdout = AlternateScreen::from(stdout);
        let backend = TermionBackend::new(stdout);
        Terminal::new(backend)?
    };

    let duration_per_frame = Duration::from_secs(1) / frames_per_second as u32;
    let (mut key_send, mut key_receive) = mpsc::channel::<Key>(1);

    // This brings blocking key-handling into the async world
    std::thread::spawn(move || -> Result<(), io::Error> {
        for key in io::stdin().keys() {
            let key = key?;
            futures::executor::block_on(key_send.send(key)).ok();
        }
        Ok(())
    });

    let render_fut = async move {
        let mut entries_buf = Vec::new();
        loop {
            let window_size = terminal.pre_render().expect("pre-render to work");
            let buf = terminal.current_buffer_mut();
            progress.sorted_snapshot(&mut entries_buf);

            entries_buf = draw_everything(entries_buf, window_size, buf);
            terminal.post_render().expect("post render to work");

            let delay = Delay::new(duration_per_frame);
            match select(delay, key_receive.next()).await {
                Either::Left(_delay_timed_out) => continue,
                Either::Right((Some(key), _delay)) => match key {
                    Key::Esc | Key::Char('q') | Key::Ctrl('c') | Key::Ctrl('[') => {
                        return ();
                    }
                    _ => continue,
                },
                _ => continue,
            };
        }
    };
    Ok(render_fut)
}

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

fn draw_everything(
    entries: Vec<(tree::Key, TreeValue)>,
    bound: Rect,
    buf: &mut Buffer,
) -> Vec<(tree::Key, TreeValue)> {
    let mut progress_pane = Block::default()
        .title("Progress Tree")
        .borders(Borders::ALL);
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
            draw_tree_prefix(&entries, buf, prefix_area)
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
            draw_progress(&entries, buf, progress_area);
        }

        if is_overflowing {
            let overflow_rect = Rect {
                y: bound.height + 1,
                height: 1,
                ..bound
            };
            draw_overflow(
                entries.iter().skip(bound.height as usize),
                buf,
                overflow_rect,
            );
        }
    }
    entries
}

fn draw_progress(entries: &[(tree::Key, TreeValue)], buf: &mut Buffer, bound: Rect) {
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
                    x: bound.x + title_spacing,
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

// TODO: put this in tui-react
fn draw_text_nowrap<'a>(
    bound: Rect,
    buf: &mut Buffer,
    t: impl AsRef<str>,
    s: impl Into<Option<Style>>,
) -> u16 {
    let s = s.into();
    let t = t.as_ref();
    let mut graphemes = t.graphemes(true);
    let mut ellipsis_candidate_x = None;
    let mut num_graphemes = 0;
    for (g, x) in graphemes.by_ref().zip(bound.left()..bound.right()) {
        num_graphemes += 1;
        let cell = buf.get_mut(x, bound.y);
        if x + 1 == bound.right() {
            ellipsis_candidate_x = Some(x);
        }
        cell.symbol = g.into();
        if let Some(s) = s {
            cell.style = s;
        }
    }
    if let (Some(_), Some(x)) = (graphemes.next(), ellipsis_candidate_x) {
        buf.get_mut(x, bound.y).symbol = "…".into();
    }
    num_graphemes
}

// TODO: put this in tui-react
fn draw_text_nowrap_fn(
    bound: Rect,
    buf: &mut Buffer,
    t: impl AsRef<str>,
    mut s: impl FnMut(&str, u16, u16) -> Style,
) {
    if bound.width == 0 {
        return;
    }
    for (g, x) in t.as_ref().graphemes(true).zip(bound.left()..bound.right()) {
        let cell = buf.get_mut(x, bound.y);
        cell.symbol = g.into();
        cell.style = s(&cell.symbol, x, bound.y);
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
        width: ((bound.width as f32 * fraction) as u16).min(bound.width),
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

fn draw_tree_prefix(entries: &[(tree::Key, TreeValue)], buf: &mut Buffer, bound: Rect) -> u16 {
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

fn draw_overflow<'a>(
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

#[derive(Default)]
struct GraphemeCountWriter(usize);

impl std::io::Write for GraphemeCountWriter {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Error> {
        self.0 += String::from_utf8_lossy(buf).graphemes(true).count();
        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<(), Error> {
        Ok(())
    }
}

/// A safe version of Rect::intersection that doesn't suffer from underflows
fn intersect(lhs: Rect, rhs: Rect) -> Rect {
    let x1 = lhs.x.max(rhs.x);
    let y1 = lhs.y.max(rhs.y);
    let x2 = lhs.right().min(rhs.right());
    let y2 = lhs.bottom().min(rhs.bottom());
    Rect {
        x: x1,
        y: y1,
        width: x2.saturating_sub(x1),
        height: y2.saturating_sub(y1),
    }
}
