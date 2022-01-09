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

    pub fn execute(&self) {
        for item in self.selected.items() {
            println!("{}", item);
        }
    }
}
