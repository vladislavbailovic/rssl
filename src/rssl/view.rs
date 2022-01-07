use super::model;
use crossterm::event::{KeyCode, KeyEvent};
use tui::{
    layout::Rect,
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph, Wrap},
};

const COLOR_BG: Color = Color::Black;
const COLOR_FG: Color = Color::Gray;

pub struct Selection {}
impl Selection {
    pub fn new() -> Selection {
        Selection {}
    }

    pub fn output(&self, model: &model::List, size: &Rect) -> Paragraph {
        let name = model.name.as_str().to_string();
        let block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(COLOR_FG).bg(COLOR_BG));
        let mut y = model.pos.get() as u16;
        if y < size.height - 3{
            y = 0;
        } else {
            y -= size.height - 3;
        }
        Paragraph::new(self.get_styled_content(model))
            .block(block.title(format!(" {} ", name)))
            .scroll((y as u16, 0))
            .wrap(Wrap { trim: false })
    }

    pub fn handle(&self, key: KeyEvent, model: &mut model::List) -> bool {
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
            _ => {},
        };
        false
    }

    fn get_styled_content(&self, model: &model::List) -> Vec<Spans> {
        let mut styled = Vec::new();
        for (idx, line) in model.items().iter().enumerate() {
            let mut style = Style::default();
            if idx == model.pos.get() {
                style = style.bg(COLOR_FG).fg(COLOR_BG);
            }
            let line = line.as_str().to_string();
            styled.push(Spans::from(vec![Span::styled(line, style)]));
        }
        styled
    }
}

pub struct Filter {}
impl Filter {
    pub fn new() -> Filter {
        Filter {}
    }

    pub fn output(&self, model: &model::Filter) -> Paragraph {
        let mut output = model.pattern().to_string();
        if model.pos() >= output.len() {
            output += "_";
        }

        let mut editable = Vec::new();
        for (idx, c) in output.chars().enumerate() {
            let mut style = Style::default();
            if idx == model.pos() {
                style = style.bg(COLOR_FG).fg(COLOR_BG);
            }
            editable.push(Span::styled(String::from(c), style));
        }

        let block = Block::default()
            .style(Style::default().fg(COLOR_FG).bg(COLOR_BG));

        Paragraph::new(Spans::from(editable))
            .block(block)
            .wrap(Wrap { trim: false })
    }

    pub fn handle(&self, key: KeyEvent, model: &mut model::Filter) -> bool {
        match key {
            KeyEvent {
                code: KeyCode::Char(c),
                modifiers: _,
            } => {
                model.push(c);
            }

            KeyEvent {
                code: KeyCode::Backspace,
                modifiers: _,
            } => {
                model.backspace();
            }

            KeyEvent {
                code: KeyCode::Left,
                modifiers: _,
            } => {
                model.left();
            }

            KeyEvent {
                code: KeyCode::Right,
                modifiers: _,
            } => {
                model.right();
            }

            _ => (),
        };
        false
    }
}
