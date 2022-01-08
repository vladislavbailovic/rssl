use super::{super::model, *};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub fn handle(key: KeyEvent, model: &mut model::Filter) -> Message {
    match key {
        KeyEvent {
            code: KeyCode::Char(c),
            modifiers: KeyModifiers::NONE | KeyModifiers::SHIFT,
        } => {
            model.push(c);
            return Message::Filter;
        }

        KeyEvent {
            code: KeyCode::Backspace,
            modifiers: _,
        } => {
            model.backspace();
            return Message::Filter;
        }

        KeyEvent {
            code: KeyCode::Left,
            modifiers: KeyModifiers::NONE,
        } => {
            model.left();
        }

        KeyEvent {
            code: KeyCode::Right,
            modifiers: KeyModifiers::NONE,
        } => {
            model.right();
        }

        _ => (),
    };
    Message::Empty
}
