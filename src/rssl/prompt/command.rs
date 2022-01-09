use super::super::model::Cursor;
use super::Prompt;

pub struct Command {
    command: String,
    pos: Cursor,
}

impl Command {
    pub fn new() -> Self {
        Self {
            command: String::new(),
            pos: Cursor::new(),
        }
    }
}

impl Prompt for Command {
    fn cursor_mut(&mut self) -> &mut Cursor {
        &mut self.pos
    }
    fn cursor(&self) -> &Cursor {
        &self.pos
    }
    fn source_mut(&mut self) -> &mut String {
        &mut self.command
    }
    fn source(&self) -> &String {
        &self.command
    }
}
