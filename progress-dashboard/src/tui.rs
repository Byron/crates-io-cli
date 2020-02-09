use crate::{
    tree::{self, TreeRoot},
    Progress, TreeValue,
};
use futures_timer::Delay;

use futures::{channel::mpsc, future::select, future::Either, SinkExt, StreamExt};
use std::{fmt, io, time::Duration};
use termion::event::Key;
use termion::{input::TermRead, raw::IntoRawMode, screen::AlternateScreen};
use tui::{
    backend::TermionBackend,
    buffer::Buffer,
    layout::Rect,
    widgets::{Block, Borders, Widget},
    widgets::{Paragraph, Text},
};
use tui_react::Terminal;

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

            draw_everything(entries_buf.drain(..), window_size, buf);
            terminal.post_render().expect("post render to work");

            let delay = Delay::new(duration_per_frame);
            match select(delay, key_receive.next()).await {
                Either::Left(_delay_timed_out) => continue,
                Either::Right((Some(key), _delay)) => match key {
                    Key::Esc | Key::Ctrl('c') | Key::Ctrl('[') => {
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

struct ProgressFormat<'a>(&'a Option<Progress>);

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
            None => Ok(()),
        }
    }
}

fn draw_everything(
    entries: impl IntoIterator<Item = (tree::Key, TreeValue)>,
    window_size: Rect,
    buf: &mut Buffer,
) {
    let mut progress_pane = Block::default()
        .title("Progress Tree")
        .borders(Borders::ALL);
    progress_pane.draw(window_size, buf);
    let current = progress_pane.inner(window_size);

    for (line, (key, value)) in entries
        .into_iter()
        .take(current.height as usize)
        .enumerate()
    {
        let tree_prefix = Text::Raw(format!("{:width$}", ' ', width = key.level() as usize).into());
        let progress = Text::Raw(
            format!(
                "{label} -- {progress}",
                label = value.title,
                progress = ProgressFormat(&value.progress)
            )
            .into(),
        );
        let line_rect = Rect {
            y: line as u16,
            height: 1,
            ..current
        };
        Paragraph::new([tree_prefix, progress].iter()).draw(line_rect, buf);
    }
}
