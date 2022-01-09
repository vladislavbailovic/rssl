use super::{super::prompt, COLOR_BG, COLOR_FG};
use tui::{
    style::Style,
    text::{Span, Spans},
    widgets::{Block, Paragraph, Wrap},
};

pub fn output(model: &dyn prompt::Prompt) -> Paragraph {
    let mut output = model.source().to_string();
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

    let block = Block::default().style(Style::default().fg(COLOR_FG).bg(COLOR_BG));

    Paragraph::new(Spans::from(editable))
        .block(block)
        .wrap(Wrap { trim: false })
}
