use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};

mod actions;
mod model;
mod view;

pub struct Rssl {
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
                // TODO: <Tab> to toggle between source/filter and selection/action pages
                _ => {
                    if let actions::Message::Filter =
                        actions::filter::handle(key, self.list.filter_mut())
                    {
                        self.list.apply_filter();
                    }
                    if let actions::Message::Item(what) =
                        actions::catalog::handle(key, &mut self.list)
                    {
                        self.selected.push(what);
                    }
                    false
                }
            };
        }
        false
    }
}
