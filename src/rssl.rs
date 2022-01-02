use crossterm::event::{self, Event, KeyEvent, KeyCode, KeyModifiers};
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    Frame,
};

mod model;
mod view;

pub struct Rssl {
    list: model::List,
    selection: view::Selection,
}
impl Rssl {
    pub fn new() -> Self {
        let list = model::List::new(
            "A List",
            vec![ "test".to_string(), "2nd line".to_string() ]
        );
        let sel = view::Selection::new();
        Self{ list, selection: sel }
    }

    pub fn draw<B: Backend>(&self, frame: &mut Frame<B>) {
        let frame_size = frame.size();
        let parts = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(2), Constraint::Length(2)].as_ref())
            .split(frame_size);

        frame.render_widget(self.selection.output(&self.list), parts[0]);
    }

    pub fn handle(&mut self) -> bool {
        if let Ok(Event::Key(key)) = event::read() {
            match key {
                KeyEvent{
                    code: KeyCode::Char('q'),
                    modifiers: KeyModifiers::CONTROL
                } => return true,
                _ => return self.selection.handle(key, &mut self.list),
            };
        }
        false
    }
}
