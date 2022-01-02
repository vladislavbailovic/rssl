use crossterm::event::{self, Event, KeyEvent, KeyCode, KeyModifiers};
use tui::{
    layout::{Constraint, Direction, Layout, Rect},
    buffer::Buffer,
    widgets::Widget,
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

impl Widget for &Rssl {

    fn render(self, area: Rect, buf: &mut Buffer) {
        let parts = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(2), Constraint::Length(2)].as_ref())
            .split(area);
        self.selection.output(&self.list).render(parts[0], buf);
    }

}
