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

    pub fn output<'a>(&self) -> Paragraph<'a> {
        let name = self.model.name.as_str().to_string();
        let mut styled = Vec::new();
        for (idx, line) in self.model.items.iter().enumerate() {
            let mut style = Style::default();
            let line = line.as_str().to_string();
            styled.push(Spans::from(vec![Span::styled(line, style)]));
        }
        Paragraph::new(styled)
            .block(get_block().title(name))
            // .scroll((state.source.get_pos() as u16, 0))
            .wrap(Wrap { trim: false })
    }
}
