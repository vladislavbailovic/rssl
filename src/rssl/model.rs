pub struct Cursor {
    current: usize,
    max: usize,
}
impl Cursor {
    pub fn get(&self) -> usize {
        self.current
    }
    pub fn max(&self) -> usize {
        self.max
    }
    pub fn set_max(&mut self, max: usize) {
        self.max = max;
    }

    pub fn set(&mut self, pos: usize) -> bool {
        if pos < self.max {
            self.current = pos;
            return true;
        }
        false
    }
    pub fn prev(&mut self) -> bool {
        if self.current > 0 {
            self.current -= 1;
            return true;
        }
        false
    }
    pub fn next(&mut self) -> bool {
        if self.current < self.max {
            self.current += 1;
            return true;
        }
        false
    }
}

pub struct Items {
    items: Vec<String>,
    filter: Filter,
}
impl Items {
    pub fn iter(&self) -> std::slice::Iter<String> {
        self.items.iter()
    }
}

pub struct List {
    pub name: String,
    pub pos: Cursor,

    items: Items,
}
impl List {
    pub fn new(name: &str, items: Vec<String>) -> List {
        let len = items.len() - 1;
        List {
            name: name.to_string(),
            items: Items {
                items,
                filter: Filter::new(),
            },
            pos: Cursor {
                current: 0,
                max: len,
            },
        }
    }

    pub fn items(&self) -> &Items {
        &self.items
    }

    pub fn filter(&self) -> &Filter {
        &self.items.filter
    }

    pub fn filter_mut(&mut self) -> &mut Filter {
        &mut self.items.filter
    }
}

pub struct Filter {
    pattern: String,
    pos: Cursor,
}
impl Filter {
    pub fn new() -> Filter {
        Filter {
            pattern: String::new(),
            pos: Cursor { current: 0, max: 0 },
        }
    }
    pub fn pos(&self) -> usize {
        self.pos.get()
    }
    pub fn pattern(&self) -> &str {
        &self.pattern
    }
    pub fn push(&mut self, c: char) {
        self.pattern.push(c);
        self.pos.set_max(self.pattern.len());
        self.pos.next();
    }
    pub fn backspace(&mut self) {
        if self.pos.get() <= self.pattern.len() {
            if self.pos.prev() {
                self.pattern.remove(self.pos.get());
            }
        }
    }
    pub fn right(&mut self) {
        self.pos.next();
    }
    pub fn left(&mut self) {
        self.pos.prev();
    }
}

pub enum Source {
    Static(String),
    Filelist(String),
    Command(String),
}
impl Source {
    pub fn load(&self, title: &str) -> List {
        match self {
            Source::Static(content) => {
                let items: Vec<String> = content.split('\n').map(String::from).collect();
                List::new(title, items)
            }
            Source::Filelist(path) => {
                let cmd = Source::Command(path.to_string());
                cmd.load(title)
            }
            Source::Command(cmd) => {
                let res = filelist(cmd);
                let stat = Source::Static(res);
                stat.load(title)
            }
        }
    }
}

use std::process::Command;
fn filelist(command: &str) -> String {
    let command = Command::new("find")
        .arg(command)
        .output()
        .expect("Command execution failed");
    let result = String::from_utf8(command.stdout).expect("Invalid stdout");
    if result.is_empty() {
        return String::from_utf8(command.stderr).expect("Invalid stderr");
    }
    result
}
