use crate::tree::TreeRoot;
use futures_timer::Delay;

use std::io;
use std::time::Duration;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
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
    let (_sender, receiver) = futures::channel::oneshot::channel::<()>();

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
            Delay::new(duration_per_frame).await;
        }
    };
    Ok((render_fut, receiver))
}
