use super::{super::model, COLOR_BG, COLOR_FG};
use tui::{
    layout::Rect,
    style::Style,
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph, Wrap},
};

pub fn output<'a>(model: &'a dyn model::Listlike, size: &Rect) -> Paragraph<'a> {
    let name = model.name().as_str().to_string();
    let block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(COLOR_FG).bg(COLOR_BG));
    let mut y = model.pos().get() as u16;
    if y < size.height - 3 {
        y = 0;
    } else {
        y -= size.height - 3;
    }
    Paragraph::new(get_styled_content(model))
        .block(block.title(format!(" {} ", name)))
        .scroll((y as u16, 0))
        .wrap(Wrap { trim: false })
}

fn get_styled_content(model: &dyn model::Listlike) -> Vec<Spans> {
    let mut styled = Vec::new();
    for (idx, line) in model.items().iter().enumerate() {
        let mut style = Style::default();
        if idx == model.pos().get() {
            style = style.bg(COLOR_FG).fg(COLOR_BG);
        }
        let line = line.as_str().to_string();
        styled.push(Spans::from(vec![Span::styled(line, style)]));
    }
    styled
}
