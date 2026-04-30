use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{StatefulWidget, Widget},
};

use crate::app::{
    App, AppContext,
    ui::{game::Game, stats::Stats},
};

pub mod game;
pub mod stats;

pub struct Ui {}

impl Ui {
    pub fn new() -> Self {
        Self {}
    }
}

impl StatefulWidget for Ui {
    type State = App;
    fn render(self, area: Rect, buf: &mut Buffer, app: &mut Self::State) {
        let area = area.centered_vertically(ratatui::layout::Constraint::Length(10));
        match app.ctx {
            AppContext::Game => {
                Game::new(app).render(area, buf);
            }
            AppContext::Stats => {
                Stats::new(app).render(area, buf);
            }
            _ => {}
        }
    }
}
