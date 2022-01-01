use crossterm::event::{self, Event};
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

const COLOR_BG: Color = Color::Black;
const COLOR_FG: Color = Color::Gray;

pub fn draw<B: Backend>(frame: &mut Frame<B>) {
    let frame_size = frame.size();
    let parts = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(2), Constraint::Length(2)].as_ref())
        .split(frame_size);

    let block = Block::default().title("test")
        .borders(Borders::ALL)
        .style(Style::default().fg(COLOR_FG).bg(COLOR_BG));
    let source_output = Paragraph::new("test".to_string())
        .block(block)
        // .scroll((state.source.get_pos() as u16, 0))
        .wrap(Wrap { trim: false });
    frame.render_widget(source_output, parts[0]);
}

pub fn handle() -> bool {
    if let Ok(Event::Key(key)) = event::read() {
        match key {
            _ => return true,
        };
    }
    false
}
