use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};

mod actions;
mod model;
mod view;

pub struct Rssl {
    active: view::Pane,
    list: model::List,
    pub selected: Vec<String>,
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
        Self {
            active: view::Pane::Catalog,
            list,
            selected: Vec::new(),
        }
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

    fn handle_selection(&mut self, _key: KeyEvent) {
        todo!("Selection event handling");
    }
}
