pub mod catalog;
pub mod prompt;

pub enum Message {
    Empty,
    Filter,
    Item(String),
}
