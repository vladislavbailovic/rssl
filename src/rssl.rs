use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use tui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::Widget,
};

mod model;
mod view;

pub struct Rssl {
    list: model::List,
    selection: view::Selection,
}

impl Default for Rssl {
    fn default() -> Self {
        Self::new()
    }
}

impl Rssl {
    pub fn new() -> Self {
        // let source = model::Source::Static("lines\nin\na\nmulti item\nlist".to_string());
        let source = model::Source::Filelist(".".to_string());
        let list = source.load("static list");
        let sel = view::Selection::new();
        Self {
            list,
            selection: sel,
        }
    }

    pub fn handle(&mut self) -> bool {
        if let Ok(Event::Key(key)) = event::read() {
            match key {
                KeyEvent {
                    code: KeyCode::Char('q'),
                    modifiers: KeyModifiers::CONTROL,
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
