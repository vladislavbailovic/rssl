pub mod filter;
pub use filter::Filter;

pub trait Prompt {
    fn pos(&self) -> usize;
    fn source(&self) -> &str;
    fn push(&mut self, c: char);
    fn backspace(&mut self);
    fn right(&mut self);
    fn left(&mut self);
}
