use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};

mod actions;
mod cli;
mod prompt;
mod view;

mod model;

use model::Catalog;

pub struct Rssl {
    active: view::Pane,
    list: model::FilteredList,
    selected: model::List,
    command: prompt::Command,
}

impl Rssl {
    pub fn new(args: Vec<String>) -> Self {
        let cli = cli::parse(args);
        let selected = model::List::new("selection", Vec::new());
        Self {
            active: view::Pane::Catalog,
            list: cli.source().load("TODO: come up with source naming"),
            selected,
            command: prompt::Command::new(),
        }
    }

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
        if let actions::Message::Filter = actions::prompt::handle(key, self.list.filter_mut()) {
            self.list.apply_filter();
        }

        if let actions::Message::Item(what) = actions::catalog::handle(key, &mut self.list) {
            self.selected.push(what);
        }
    }

    pub fn execute(&self) {
        for item in self.selected.items() {
            println!("{}", item);
        }
    }

    fn handle_selection(&mut self, key: KeyEvent) {
        actions::prompt::handle(key, &mut self.command);
        if let actions::Message::Item(what) = actions::catalog::handle(key, &mut self.selected) {
            self.selected.remove(what);
        }
    }
}
