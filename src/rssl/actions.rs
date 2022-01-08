pub mod catalog;
pub mod filter;

pub enum Message {
    Empty,
    Filter,
    Item(String),
}
