use crossterm::event::{self, Event};
use tui::{backend::Backend, Frame};

pub fn draw<B: Backend>(frame: &mut Frame<B>) {}

pub fn handle() -> bool {
    if let Ok(Event::Key(key)) = event::read() {
        match key {
            _ => return true,
        };
    }
    false
}
