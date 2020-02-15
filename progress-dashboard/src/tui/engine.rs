use crate::tree::TreeRoot;
use futures_timer::Delay;

use crate::tui::draw;
use futures::task::Poll;
use futures::{channel::mpsc, FutureExt, SinkExt, StreamExt};
use std::{io, time::Duration};
use termion::{event::Key, input::TermRead, raw::IntoRawMode, screen::AlternateScreen};
use tui::backend::TermionBackend;
use tui_react::Terminal;

#[derive(Clone)]
pub struct Config {
    pub title: String,
    pub frames_per_second: f32,
}

fn ticker(dur: Duration) -> impl futures::Stream<Item = ()> {
    let mut delay = Delay::new(dur);
    futures::stream::poll_fn(move |ctx| {
        let res = delay.poll_unpin(ctx);
        match res {
            Poll::Pending => Poll::Pending,
            Poll::Ready(_) => {
                delay.reset(dur);
                Poll::Ready(Some(()))
            }
        }
    })
}

enum Event {
    Tick,
    Input(Key),
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
    let (mut key_send, key_receive) = mpsc::channel::<Key>(1);

    // This brings blocking key-handling into the async world
    std::thread::spawn(move || -> Result<(), io::Error> {
        for key in io::stdin().keys() {
            let key = key?;
            futures::executor::block_on(key_send.send(key)).ok();
        }
        Ok(())
    });

    let render_fut = async move {
        let mut entries = Vec::with_capacity(progress.num_tasks());
        let mut messages = Vec::with_capacity(progress.messages_capacity());
        let mut events = futures::stream::select(
            ticker(duration_per_frame).map(|_| Event::Tick),
            key_receive.map(|key| Event::Input(key)),
        );

        while let Some(event) = events.next().await {
            match event {
                Event::Tick => {
                    let window_size = terminal.pre_render().expect("pre-render to work");
                    let buf = terminal.current_buffer_mut();
                    progress.sorted_snapshot(&mut entries);
                    progress.copy_messages(&mut messages);

                    draw::all(
                        &title,
                        duration_per_frame,
                        &entries,
                        &messages,
                        window_size,
                        buf,
                    );
                    terminal.post_render().expect("post render to work");
                }
                Event::Input(key) => match key {
                    Key::Esc | Key::Char('q') | Key::Ctrl('c') | Key::Ctrl('[') => {
                        break;
                    }
                    _ => {}
                },
            }
        }
        ()
    };
    Ok(render_fut)
}
