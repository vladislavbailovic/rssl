use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{env, io, process};
use tui::{
    backend::{Backend, CrosstermBackend},
    Frame, Terminal,
};

pub mod rssl;

fn main() {
    let terminal = bootstrap();
    exec(terminal);
}

fn draw<B: Backend>(rssl: &rssl::Rssl, frame: &mut Frame<B>) {
    frame.render_widget(rssl, frame.size());
}

fn exec<B: Backend>(mut terminal: Terminal<B>) {
    let mut r = rssl::Rssl::new(env::args().collect());
    loop {
        terminal
            .draw(|frame| draw(&r, frame))
            .expect("Could not draw UI");
        if r.handle() {
            break;
        }
    }
    terminal.clear().expect("Unable to clear the terminal");
    terminal
        .show_cursor()
        .expect("Unable to re-show the cursor");
    disable_raw_mode().expect("Could not disable raw mode");
    execute!(io::stdout(), LeaveAlternateScreen).expect("Unable to leave alternate screen");

    r.execute();

    process::exit(0)
}

fn bootstrap() -> Terminal<impl Backend> {
    execute!(io::stdout(), EnterAlternateScreen).expect("Unable to enter alternate screen");
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend).expect("Unable to bootstrap terminal");

    enable_raw_mode().expect("Could not enable raw mode");
    terminal
}
