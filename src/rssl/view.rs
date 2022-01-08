pub mod catalog;
pub mod filter;

use tui::style::Color;

const COLOR_BG: Color = Color::Black;
const COLOR_FG: Color = Color::Gray;

pub enum Pane {
    Catalog,
    Selection,
}

use super::Rssl;
use tui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::Widget,
};

impl Widget for &Rssl {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let parts = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(2), Constraint::Length(1)].as_ref())
            .split(area);
        match self.active {
            Pane::Catalog => {
                catalog::output(&self.list, &parts[0]).render(parts[0], buf);
                filter::output(self.list.filter()).render(parts[1], buf);
            }
            Pane::Selection => {}
        };
    }
}
