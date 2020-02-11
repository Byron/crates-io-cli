mod tasks;
mod utils;

use crate::{
    tree::{self, TreeRoot},
    TreeValue,
};
use futures_timer::Delay;

use crate::tui::utils::intersect;
use futures::{channel::mpsc, future::select, future::Either, SinkExt, StreamExt};
use std::{io, time::Duration};
use termion::{event::Key, input::TermRead, raw::IntoRawMode, screen::AlternateScreen};
use tui::{
    backend::TermionBackend,
    buffer::Buffer,
    layout::Rect,
    widgets::{Block, Borders, Widget},
};
use tui_react::Terminal;

#[derive(Clone)]
pub struct Config {
    pub title: String,
    pub frames_per_second: u8,
}

pub fn render(
    progress: TreeRoot,
    Config {
        title,
        frames_per_second,
    }: Config,
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

            entries_buf = draw_everything(&title, entries_buf, window_size, buf);
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

fn draw_everything(
    title: impl AsRef<str>,
    entries: Vec<(tree::Key, TreeValue)>,
    bound: Rect,
    buf: &mut Buffer,
) -> Vec<(tree::Key, TreeValue)> {
    let mut progress_pane = Block::default().title(title.as_ref()).borders(Borders::ALL);
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
            tasks::draw_tree_prefix(&entries, buf, prefix_area)
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
            tasks::draw_progress(&entries, buf, progress_area);
        }

        if is_overflowing {
            let overflow_rect = Rect {
                y: bound.height + 1,
                height: 1,
                ..bound
            };
            tasks::draw_overflow(
                entries.iter().skip(bound.height as usize),
                buf,
                overflow_rect,
            );
        }
    }
    entries
}
