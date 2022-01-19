use super::prompt::{Filter, Prompt};

pub struct Cursor {
    current: usize,
    max: usize,
}
impl Cursor {
    pub fn new() -> Self {
        Self { current: 0, max: 0 }
    }

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

pub trait Catalog {
    fn name(&self) -> &String;
    fn pos(&self) -> &Cursor;
    fn pos_mut(&mut self) -> &mut Cursor;
    fn items(&self) -> &Vec<String>;
    fn current(&self) -> String {
        if self.items().is_empty() {
            return "".to_string();
        }
        self.items()[self.pos().get()].as_str().to_string()
    }
}

pub struct FilteredList {
    name: String,
    pos: Cursor,

    filter: Filter,
    items: Vec<String>,
    filtered: Vec<String>,
}
impl FilteredList {
    pub fn new(name: &str, items: Vec<String>) -> Self {
        let len = items.len() - 1;
        Self {
            name: name.to_string(),
            items,
            filter: Filter::new(),
            pos: Cursor {
                current: 0,
                max: len,
            },
            filtered: Vec::new(),
        }
    }

    pub fn apply_filter(&mut self) {
        self.filtered = Vec::new();
        for item in self.items.iter() {
            if item.contains(self.filter.source()) {
                self.filtered.push(item.to_string());
            }
        }
        if !self.filtered.is_empty() {
            if self.pos.get() >= self.filtered.len() - 1 {
                self.pos.set(self.filtered.len() - 1);
            }
            self.pos.set_max(self.filtered.len() - 1);
        } else {
            self.pos.set(0);
            self.pos.set_max(0);
        }
    }

    pub fn filter(&self) -> &Filter {
        &self.filter
    }

    pub fn filter_mut(&mut self) -> &mut Filter {
        &mut self.filter
    }
}

impl Catalog for FilteredList {
    fn name(&self) -> &String {
        &self.name
    }

    fn pos(&self) -> &Cursor {
        &self.pos
    }

    fn pos_mut(&mut self) -> &mut Cursor {
        &mut self.pos
    }

    fn items(&self) -> &Vec<String> {
        if !self.filter.source().is_empty() {
            return &self.filtered;
        }
        &self.items
    }
}

pub struct List {
    name: String,
    pos: Cursor,
    items: Vec<String>,
}
impl List {
    pub fn new(name: &str, items: Vec<String>) -> Self {
        let mut len = 0;
        if !items.is_empty() {
            len = items.len() - 1;
        }
        Self {
            name: name.to_string(),
            items,
            pos: Cursor {
                current: 0,
                max: len,
            },
        }
    }
}

impl List {
    pub fn push(&mut self, item: String) {
        if !self.items.contains(&item) {
            self.items.push(item);
            if !self.items.is_empty() {
                self.pos.set_max(self.items.len() - 1);
            }
        }
    }

    pub fn remove(&mut self, what: String) {
        for (idx, item) in self.items.iter().enumerate() {
            if item != &what {
                continue;
            }
            self.items.remove(idx);
            if !self.items.is_empty() {
                self.pos.set_max(self.items.len() - 1);
            }
            break;
        }
    }
}

impl Catalog for List {
    fn name(&self) -> &String {
        &self.name
    }

    fn pos(&self) -> &Cursor {
        &self.pos
    }

    fn pos_mut(&mut self) -> &mut Cursor {
        &mut self.pos
    }

    fn items(&self) -> &Vec<String> {
        &self.items
    }
}
