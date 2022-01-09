use super::Prompt;
use super::super::model::Cursor;

pub struct Filter {
    pattern: String,
    pos: Cursor,
}

impl Filter {
    pub fn new() -> Self {
        Self {
            pattern: String::new(),
            pos: Cursor::new(),
        }
    }
}

impl Prompt for Filter {
    fn pos(&self) -> usize {
        self.pos.get()
    }
    fn source(&self) -> &str {
        &self.pattern
    }
    fn push(&mut self, c: char) {
        self.pattern.push(c);
        self.pos.set_max(self.pattern.len());
        self.pos.next();
    }
    fn backspace(&mut self) {
        if self.pos.get() <= self.pattern.len() && self.pos.prev() {
            self.pattern.remove(self.pos.get());
        }
    }
    fn right(&mut self) {
        self.pos.next();
    }
    fn left(&mut self) {
        self.pos.prev();
    }
}
