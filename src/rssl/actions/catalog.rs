use super::{super::model, *};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub fn handle(key: KeyEvent, model: &mut dyn model::Catalog) -> Message {
    match key {
        KeyEvent {
            code: KeyCode::Up,
            modifiers: _,
        } => {
            model.pos_mut().prev();
        }

        KeyEvent {
            code: KeyCode::PageUp,
            modifiers: _,
        } => {
            for _ in 0..10 {
                model.pos_mut().prev();
            }
        }

        KeyEvent {
            code: KeyCode::Home,
            modifiers: _,
        } => {
            model.pos_mut().set(0);
        }

        KeyEvent {
            code: KeyCode::Down,
            modifiers: _,
        } => {
            model.pos_mut().next();
        }

        KeyEvent {
            code: KeyCode::PageDown,
            modifiers: _,
        } => {
            for _ in 0..10 {
                model.pos_mut().next();
            }
        }

        KeyEvent {
            code: KeyCode::End,
            modifiers: _,
        } => {
            let pos = model.pos().max();
            model.pos_mut().set(pos - 1);
        }

        KeyEvent {
            code: KeyCode::Char(' '),
            modifiers: _,
        } => {
            let item = model.current();
            return Message::Item(item);
        }
        _ => {}
    };
    Message::Empty
}
