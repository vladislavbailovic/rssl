use super::{super::model, *};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub fn handle(key: KeyEvent, model: &mut model::List) -> Message {
    match key {
        KeyEvent {
            code: KeyCode::Up,
            modifiers: _,
        } => {
            model.pos.prev();
        }

        KeyEvent {
            code: KeyCode::PageUp,
            modifiers: _,
        } => {
            for _ in 0..10 {
                model.pos.prev();
            }
        }

        KeyEvent {
            code: KeyCode::Home,
            modifiers: _,
        } => {
            model.pos.set(0);
        }

        KeyEvent {
            code: KeyCode::Down,
            modifiers: _,
        } => {
            model.pos.next();
        }

        KeyEvent {
            code: KeyCode::PageDown,
            modifiers: _,
        } => {
            for _ in 0..10 {
                model.pos.next();
            }
        }

        KeyEvent {
            code: KeyCode::End,
            modifiers: _,
        } => {
            model.pos.set(model.pos.max() - 1);
        }

        KeyEvent {
            code: KeyCode::Char(' '),
            modifiers: KeyModifiers::CONTROL,
        } => {
            let item = model.current();
            return Message::Item(item);
        }
        _ => {}
    };
    Message::Empty
}
