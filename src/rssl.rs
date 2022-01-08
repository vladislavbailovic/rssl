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
    filter: view::Filter,
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
        let filter = view::Filter::new();
        Self {
            list,
            filter,
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
                // TODO: <ctrl+space> to (un)select an item
                // TODO: <Tab> to toggle between source/filter and selection/action pages
                _ => {
                    self.filter.handle(key, self.list.filter_mut());
                    self.list.apply_filter();
                    self.selection.handle(key, &mut self.list);
                    false
                }
            };
        }
        false
    }
}

impl Widget for &Rssl {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let parts = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(2), Constraint::Length(1)].as_ref())
            .split(area);
        self.selection
            .output(&self.list, &parts[0])
            .render(parts[0], buf);
        self.filter.output(self.list.filter()).render(parts[1], buf);
    }
}
