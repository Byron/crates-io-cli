use crate::tree::TreeRoot;
use futures_timer::Delay;

use crate::tui::draw;
use futures::{channel::mpsc, future::select, future::Either, SinkExt, StreamExt};
use std::{io, time::Duration};
use termion::{event::Key, input::TermRead, raw::IntoRawMode, screen::AlternateScreen};
use tui::backend::TermionBackend;
use tui_react::Terminal;

#[derive(Clone)]
pub struct Config {
    pub title: String,
    pub frames_per_second: f32,
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

    let duration_per_frame = Duration::from_secs_f32(1.0 / frames_per_second);
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
        let mut messages = Vec::new();
        loop {
            let window_size = terminal.pre_render().expect("pre-render to work");
            let buf = terminal.current_buffer_mut();
            progress.sorted_snapshot(&mut entries_buf);
            progress.copy_messages(&mut messages);

            entries_buf = draw::all(&title, duration_per_frame, entries_buf, window_size, buf);
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
