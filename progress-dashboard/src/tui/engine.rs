use crate::{tree::TreeRoot, tui::draw, tui::ticker};

use futures::{channel::mpsc, SinkExt, StreamExt};
use std::{io, time::Duration};
use termion::{event::Key, input::TermRead, raw::IntoRawMode, screen::AlternateScreen};
use tui::{backend::TermionBackend, layout::Rect};
use tui_react::Terminal;

#[derive(Clone)]
pub struct Config {
    pub title: String,
    pub frames_per_second: f32,
}

pub enum Event {
    Tick,
    Input(Key),
    SetWindowSize(Rect),
}

#[derive(Default)]
pub struct State {
    pub title: String,
    pub task_offset: u16,
    pub message_offset: u16,
    pub user_provided_window_size: Option<Rect>,
}

pub fn render_with_input(
    progress: TreeRoot,
    Config {
        title,
        frames_per_second,
    }: Config,
    events: impl futures::Stream<Item = Event> + Send,
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
        let mut state = State {
            title,
            ..State::default()
        };
        let mut entries = Vec::with_capacity(progress.num_tasks());
        let mut messages = Vec::with_capacity(progress.messages_capacity());
        let mut events = futures::stream::select_all(vec![
            ticker(duration_per_frame).map(|_| Event::Tick).boxed(),
            key_receive.map(|key| Event::Input(key)).boxed(),
            events.boxed(),
        ]);

        while let Some(event) = events.next().await {
            match event {
                Event::Tick => {
                    let window_size = terminal.pre_render().expect("pre-render to work");
                    let window_size = state.user_provided_window_size.unwrap_or(window_size);
                    let buf = terminal.current_buffer_mut();
                    progress.sorted_snapshot(&mut entries);
                    progress.copy_messages(&mut messages);

                    draw::all(
                        &state,
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
                    Key::Char('J') => state.message_offset = state.message_offset.saturating_add(1),
                    Key::Char('j') => state.task_offset = state.task_offset.saturating_add(1),
                    Key::Char('K') => state.message_offset = state.message_offset.saturating_sub(1),
                    Key::Char('k') => state.task_offset = state.task_offset.saturating_sub(1),
                    _ => {}
                },
                Event::SetWindowSize(bound) => state.user_provided_window_size = Some(bound),
            }
        }
    };
    Ok(render_fut)
}

pub fn render(
    progress: TreeRoot,
    config: Config,
) -> Result<impl std::future::Future<Output = ()>, std::io::Error> {
    return render_with_input(progress, config, futures::stream::pending());
}
