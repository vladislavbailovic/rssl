use crossterm::event::{KeyEvent, KeyCode};
use tui::{
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph, Wrap},
};
use super::model;

const COLOR_BG: Color = Color::Black;
const COLOR_FG: Color = Color::Gray;

fn get_block<'a>() -> Block<'a> {
    Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(COLOR_FG).bg(COLOR_BG))
}

pub struct Selection {
}
impl Selection {
    pub fn new() -> Selection {
        Selection{ }
    }

    pub fn output(&self, model: &model::List) -> Paragraph {
        let name = model.name.as_str().to_string();
        Paragraph::new(self.get_styled_content(model))
            .block(get_block().title(name))
            // .scroll((model.pos.get() as u16, 0))
            .wrap(Wrap { trim: false })
    }

    pub fn handle(&self, key: KeyEvent, model: &mut model::List) -> bool {
        match key {
            KeyEvent{
                code: KeyCode::Up,
                modifiers
            } => model.pos.prev(),
            KeyEvent{
                code: KeyCode::Down,
                modifiers
            } => model.pos.next(),
            _ => false,
        };
        false
    }

    fn get_styled_content(&self, model: &model::List) -> Vec<Spans> {
        let mut styled = Vec::new();
        for (idx, line) in model.items.iter().enumerate() {
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