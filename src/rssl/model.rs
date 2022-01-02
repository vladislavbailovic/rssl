pub struct Cursor {
    current: usize,
    max: usize,
}
impl Cursor {
    pub fn get(&self) -> usize {
        self.current
    }
    pub fn set(&mut self, pos: usize) -> bool {
        if pos > 0 && pos < self.max {
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
        return false;
    }
}

pub struct Items {
    items: Vec<String>,
}
impl Items {
    pub fn iter(&self) -> std::slice::Iter<String> {
        self.items.iter()
    }
}

pub struct List {
    pub name: String,
    pub items: Items,
    pub pos: Cursor,
}
impl List {
    pub fn new(name: &str, items: Vec<String>) -> List {
        let len = items.len() - 1;
        List {
            name: name.to_string(),
            items: Items { items },
            pos: Cursor{ current: 0, max: len }
        }
    }
}

pub enum Source {
    Static(String),
    Filelist(String),
}
impl Source {
    pub fn load(&self, title: &str) -> List {
        match self {
            Source::Static(content) => {
                let items: Vec<String> = content.split('\n').map(String::from).collect();
                List::new(title, items)
            }
            _ => List::new(title, vec![ "files".to_string() ]),
        }
    }
}
