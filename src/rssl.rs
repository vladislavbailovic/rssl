use crossterm::event::{self, Event};
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    Frame,
};

mod model;
mod view;

pub fn draw<B: Backend>(frame: &mut Frame<B>) {
    let frame_size = frame.size();
    let parts = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(2), Constraint::Length(2)].as_ref())
        .split(frame_size);

    let sel = view::Selection::new(model::List{
        name: "A List".to_string(),
        items: vec![ "test".to_string(), "2nd line".to_string() ]
    });
    frame.render_widget(sel.output(), parts[0]);
}

pub fn handle() -> bool {
    if let Ok(Event::Key(key)) = event::read() {
        match key {
            _ => return true,
        };
    }
    false
}
