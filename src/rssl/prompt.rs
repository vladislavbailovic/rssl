pub mod filter;
pub use filter::Filter;

pub mod command;
pub use command::Command;

use super::model::Cursor;

pub trait Prompt {
    fn cursor_mut(&mut self) -> &mut Cursor;
    fn cursor(&self) -> &Cursor;
    fn source_mut(&mut self) -> &mut String;
    fn source(&self) -> &String;

    fn pos(&self) -> usize {
        self.cursor().get()
    }

    fn push(&mut self, c: char) {
        self.source_mut().push(c);
        let max = self.source().len();
        self.cursor_mut().set_max(max);
        self.cursor_mut().next();
    }

    fn backspace(&mut self) {
        if self.cursor().get() <= self.source().len() && self.cursor_mut().prev() {
            let idx = self.cursor().get();
            self.source_mut().remove(idx);
        }
    }

    fn right(&mut self) {
        self.cursor_mut().next();
    }

    fn left(&mut self) {
        self.cursor_mut().prev();
    }
}
