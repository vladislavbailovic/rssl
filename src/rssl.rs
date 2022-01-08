use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};

mod actions;
mod model;
mod view;

use model::Listlike;

pub struct Rssl {
    active: view::Pane,
    list: model::FilteredList,
    selected: model::List,
}

impl Default for Rssl {
    fn default() -> Self {
        Self::new()
    }
}

impl Rssl {
    pub fn new() -> Self {
        let source = model::Source::Filelist(".".to_string());
        let list = source.load("static list");
        let selected = model::List::new("selection", Vec::new());
        Self {
            active: view::Pane::Catalog,
            list,
            selected,
        }
    }

    pub fn selected(&self) -> &Vec<String> {
        self.selected.items()
    }

    pub fn handle(&mut self) -> bool {
        if let Ok(Event::Key(key)) = event::read() {
            match key {
                KeyEvent {
                    code: KeyCode::Char('q'),
                    modifiers: KeyModifiers::CONTROL,
                } => return true,

                // TODO: swap active pane
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
        if let actions::Message::Filter = actions::filter::handle(key, self.list.filter_mut()) {
            self.list.apply_filter();
        }

        if let actions::Message::Item(what) = actions::catalog::handle(key, &mut self.list) {
            self.selected.push(what);
        }
    }

    fn handle_selection(&mut self, key: KeyEvent) {
        if let actions::Message::Item(what) = actions::catalog::handle(key, &mut self.selected) {
            self.selected.remove(what);
        }
    }
}
