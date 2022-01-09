use super::super::model::Cursor;
use super::Prompt;

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
    fn cursor_mut(&mut self) -> &mut Cursor {
        &mut self.pos
    }
    fn cursor(&self) -> &Cursor {
        &self.pos
    }
    fn source_mut(&mut self) -> &mut String {
        &mut self.pattern
    }
    fn source(&self) -> &String {
        &self.pattern
    }
}
