pub mod game;
pub mod stats;

use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};

use crate::app::{
    App, AppContext,
    ui::{game::Game, stats::Stats},
};

impl<'a> Widget for &App<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let area = area.centered_vertically(ratatui::layout::Constraint::Length(10));
        match self.ctx {
            AppContext::Game => {
                Game::new(self).render(area, buf);
            }
            AppContext::Stats => {
                Stats::new(self).render(area, buf);
            }
            _ => {}
        }
    }
}
