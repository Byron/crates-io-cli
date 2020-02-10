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
    window_size: Rect,
    buf: &mut Buffer,
) -> Vec<(tree::Key, TreeValue)> {
    let mut progress_pane = Block::default()
        .title("Progress Tree")
        .borders(Borders::ALL);
    progress_pane.draw(window_size, buf);
    let mut current = progress_pane.inner(window_size);
    let is_overflowing = if entries.len() > current.height as usize {
        current.height = current.height.saturating_sub(1);
        true
    } else {
        false
    };

    let column_width = current.width / 2;
    let max_prefix_width = {
        let prefix_area = Rect {
            width: column_width,
            ..current
        };
        draw_tree_prefix(&entries, buf, prefix_area).map(|l| l.min(column_width))
    };

    {
        let max_prefix_len = max_prefix_width.unwrap_or_default();
        let progress_area = Rect {
            x: current.x + max_prefix_len,
            width: current.width.saturating_sub(max_prefix_len),
            ..current
        };
        draw_progress(&entries, buf, progress_area);
    }

    if is_overflowing {
        let overflow_rect = Rect {
            y: current.height + 1,
            height: 1,
            ..current
        };
        draw_overflow(
            entries.iter().skip(current.height as usize),
            buf,
            overflow_rect,
        );
    }
    entries
}

fn draw_progress(entries: &[(tree::Key, TreeValue)], buf: &mut Buffer, bound: Rect) {
    let x_offset = 1;
    let title_spacing = 1 + 1 + 1;
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
            None => state.max(title.graphemes(true).count() + key.level() as usize + title_spacing),
            Some(_) => state,
        },
    );
    for (line, (key, TreeValue { progress, title })) in
        entries.iter().take(bound.height as usize).enumerate()
    {
        let max_width = bound.width.saturating_sub(x_offset);
        let progress_text = format!(
            " {progress}",
            progress = ProgressFormat(progress, max_width)
        );

        let y = bound.y + line as u16;
        let progress_bar_info = if let Some(fraction) = progress.and_then(|p| p.fraction()) {
            let bar_bound = Rect {
                x: bound.x + x_offset + column_line_width + 1,
                width: max_width,
                y,
                height: 1,
            };
            Some(draw_progress_bar(buf, bar_bound, fraction))
        } else {
            None
        };

        let mut progress_rect = Rect {
            x: bound.x + x_offset + column_line_width,
            y,
            height: 1,
            ..bound
        }
        .intersection(bound);
        draw_text_nowrap(progress_rect, buf, "│", None);
        progress_rect = Rect {
            x: progress_rect.x + column_line_width,
            ..progress_rect
        }
        .intersection(bound);
        if let Some(style_fn) = progress_bar_info.map(|(bound, style)| {
            move |_t: &str, x: u16, _y: u16| {
                if x < bound.right() {
                    style
                } else {
                    Style::default()
                }
            }
        }) {
            draw_text_nowrap_fn(progress_rect, buf, progress_text, style_fn);
        } else {
            draw_text_nowrap(progress_rect, buf, progress_text, None);
            // we have progress, but no upper limit
            if let Some((step, None)) = progress.as_ref().map(|p| (p.step, p.done_at.as_ref())) {
                let bar_rect = Rect {
                    x: bound.x + max_progress_label_width as u16,
                    y,
                    height: 1,
                    ..bound
                }
                .intersection(bound);
                draw_spinner(buf, bar_rect, step, line);
            }
        }

        if progress.is_none() {
            let center_rect = Rect {
                x: bound.x
                    + x_offset
                    + column_line_width
                    + (bound.width.saturating_sub(max_title_width as u16)) / 2,
                y,
                width: max_title_width as u16,
                height: 1,
            }
            .intersection(bound);
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
) {
    let s = s.into();
    for (g, x) in t.as_ref().graphemes(true).zip(bound.left()..bound.right()) {
        let cell = buf.get_mut(x, bound.y);
        cell.symbol = g.into();
        if let Some(s) = s {
            cell.style = s;
        }
    }
}

// TODO: put this in tui-react
fn draw_text_nowrap_fn(
    bound: Rect,
    buf: &mut Buffer,
    t: impl AsRef<str>,
    mut s: impl FnMut(&str, u16, u16) -> Style,
) {
    for (g, x) in t.as_ref().graphemes(true).zip(bound.left()..bound.right()) {
        let cell = buf.get_mut(x, bound.y);
        cell.symbol = g.into();
        cell.style = s(&cell.symbol, x, bound.y);
    }
}

fn draw_spinner(buf: &mut Buffer, bound: Rect, step: ProgressStep, seed: usize) {
    let x = bound.x + (step as usize + seed % bound.width as usize) as u16;
    let width = 5;
    let bound = Rect { x, width, ..bound }.intersection(bound);
    tui_react::fill_background(bound, buf, Color::White);
}

fn draw_progress_bar(buf: &mut Buffer, bound: Rect, fraction: f32) -> (Rect, Style) {
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

fn draw_tree_prefix(
    entries: &[(tree::Key, TreeValue)],
    buf: &mut Buffer,
    bound: Rect,
) -> Option<u16> {
    let mut max_prefix_len = None;
    for (line, (key, TreeValue { progress, title })) in
        entries.iter().take(bound.height as usize).enumerate()
    {
        let mut tree_prefix = format!(
            "{:>width$} {}",
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
        tree_prefix = tree_prefix
            .graphemes(true)
            .take(bound.width.saturating_sub(1) as usize)
            .collect();
        if tree_prefix.len() + 1 >= bound.width as usize {
            tree_prefix.push('…');
        }
        max_prefix_len = Some(max_prefix_len.unwrap_or(0).max(tree_prefix.len() as u16));
        let line_rect = Rect {
            y: bound.y + line as u16,
            height: 1,
            ..bound
        };
        draw_text_nowrap(line_rect, buf, tree_prefix, None);
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
