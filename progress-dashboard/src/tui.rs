use crate::tree::TreeRoot;
use futures_timer::Delay;

use futures::{future::select, future::Either, SinkExt, StreamExt};
use std::{io, time::Duration};
use termion::{input::TermRead, raw::IntoRawMode, screen::AlternateScreen};
use tui::{
    backend::TermionBackend,
    widgets::{Block, Borders, Widget},
    Terminal,
};

#[derive(Clone)]
pub struct Config {
    pub frames_per_second: u8,
}

pub fn render(
    _progress: TreeRoot,
    Config { frames_per_second }: Config,
) -> Result<
    (
        impl std::future::Future<Output = ()>,
        futures::channel::oneshot::Receiver<()>,
    ),
    std::io::Error,
> {
    let mut terminal = {
        let stdout = io::stdout().into_raw_mode()?;
        let stdout = AlternateScreen::from(stdout);
        let backend = TermionBackend::new(stdout);
        Terminal::new(backend)?
    };

    let duration_per_frame = Duration::from_secs(1) / frames_per_second as u32;
    let (_send_gui_aborted, receive_gui_aborted) = futures::channel::oneshot::channel::<()>();
    let (mut key_send, mut _key_receive) =
        futures::channel::mpsc::channel::<termion::event::Key>(1);

    std::thread::spawn(move || -> Result<(), io::Error> {
        for key in io::stdin().keys() {
            let key = key?;
            futures::executor::block_on(key_send.send(key)).ok();
        }
        Ok(())
    });

    let render_fut = async move {
        loop {
            if let Err(err) = terminal.draw(|mut f| {
                let size = f.size();
                Block::default()
                    .title("Block")
                    .borders(Borders::ALL)
                    .render(&mut f, size);
            }) {
                log::error!("{}", err);
                return ();
            }
            match select(Delay::new(duration_per_frame), _key_receive.next()).await {
                Either::Left(_delay_timed_out) => {} // redraw
                Either::Right((key, _delay)) => eprintln!("{:#?}", key),
            }
        }
    };
    Ok((render_fut, receive_gui_aborted))
}
