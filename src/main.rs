use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{io, process};
use tui::{
    backend::{Backend, CrosstermBackend},
    Frame, Terminal,
};

pub mod rssl;

fn main() {
    let terminal = bootstrap();
    exec(terminal);
}

fn exit(status: i32) {
    disable_raw_mode().expect("Could not disable raw mode");
    execute!(io::stdout(), LeaveAlternateScreen).expect("Unable to leave alternate screen");
    process::exit(status)
}

fn exec<B: Backend>(mut terminal: Terminal<B>) {
    let mut r = rssl::Rssl::new();
    loop {
        terminal
            .draw(|frame| r.draw(frame))
            .expect("Could not draw UI");
        if r.handle() {
            break;
        }
    }
    exit(0);
}

fn bootstrap() -> Terminal<impl Backend> {
    execute!(io::stdout(), EnterAlternateScreen).expect("Unable to enter alternate screen");
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend).expect("Unable to bootstrap terminal");

    enable_raw_mode().expect("Could not enable raw mode");
    terminal
}
