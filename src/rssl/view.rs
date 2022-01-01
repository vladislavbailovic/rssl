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
    model: model::List,
}
impl Selection {
    pub fn new(model: model::List) -> Selection {
        Selection{ model }
    }

    pub fn output(&self) -> Paragraph {
        let name = self.model.name.as_str().to_string();
        Paragraph::new(self.get_styled_content())
            .block(get_block().title(name))
            // .scroll((state.source.get_pos() as u16, 0))
            .wrap(Wrap { trim: false })
    }

    pub fn get_styled_content(&self) -> Vec<Spans> {
        let mut styled = Vec::new();
        for (idx, line) in self.model.items.iter().enumerate() {
            let mut style = Style::default();
            let line = line.as_str().to_string();
            styled.push(Spans::from(vec![Span::styled(line, style)]));
        }
        styled
    }
}
