use std::{io,process};
use crossterm::{
    execute,
    event::{self,Event},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{backend::{CrosstermBackend,Backend}, Terminal, Frame};

fn main() {
    let terminal = bootstrap();
    exec(terminal);
}

fn exit(status: i32) {
    disable_raw_mode().expect("Could not disable raw mode");
    execute!(io::stdout(), LeaveAlternateScreen)
        .expect("Unable to leave alternate screen");
    process::exit(status)
}

pub fn draw<B: Backend>(frame: &mut Frame<B>) {
}

fn exec<B: Backend>(mut terminal: Terminal<B>) {
    loop {
        terminal
            .draw(|frame| draw(frame))
            .expect("Could not draw UI");
        if let Ok(Event::Key(key)) = event::read() {
            match key {
                _ => return exit(161),
            };
        }
    }
}

fn bootstrap() -> Terminal<impl Backend> {
    execute!(io::stdout(), EnterAlternateScreen).expect("Unable to enter alternate screen");
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).expect("Unable to bootstrap terminal");

    enable_raw_mode().expect("Could not enable raw mode");
    terminal
}
