pub mod catalog;
pub mod prompt;

pub enum Message {
    Empty,
    Filter,
    Item(String),
}

use super::{model::Catalog, view, Rssl};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};

impl Rssl {
    pub fn handle(&mut self) -> bool {
        if let Ok(Event::Key(key)) = event::read() {
            match key {
                KeyEvent {
                    code: KeyCode::Char('q'),
                    modifiers: KeyModifiers::CONTROL,
                } => return true,

                KeyEvent {
                    code: KeyCode::Enter,
                    modifiers: _,
                } => {
                    let item = self.list.current();
                    self.selected.push(item);
                    return true;
                }

                KeyEvent {
                    code: KeyCode::Tab,
                    modifiers: KeyModifiers::NONE | KeyModifiers::SHIFT,
                } => {
                    self.active = match self.active {
                        view::Pane::Catalog => view::Pane::Selection,
                        view::Pane::Selection => view::Pane::Catalog,
                    };
                    false
                }

                _ => {
                    match self.active {
                        view::Pane::Catalog => self.handle_catalog(key),
                        view::Pane::Selection => self.handle_selection(key),
                    }
                    false
                }
            };
        }
        false
    }

    fn handle_catalog(&mut self, key: KeyEvent) {
        if let Message::Filter = prompt::handle(key, self.list.filter_mut()) {
            self.list.apply_filter();
        }

        if let Message::Item(what) = catalog::handle(key, &mut self.list) {
            self.selected.push(what);
        }
    }

    fn handle_selection(&mut self, key: KeyEvent) {
        prompt::handle(key, &mut self.command);
        if let Message::Item(what) = catalog::handle(key, &mut self.selected) {
            self.selected.remove(what);
        }
    }
}
