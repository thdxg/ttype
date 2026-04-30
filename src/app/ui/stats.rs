use crate::app::App;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Rect},
    style::{Style, Stylize},
    text::Line,
    widgets::{Block, Borders, Cell, Padding, Row, Table, Widget},
};

pub struct Stats<'a> {
    app: &'a App,
}

impl<'a> Stats<'a> {
    pub fn new(app: &'a App) -> Self {
        Self { app }
    }
}

impl<'a> Widget for Stats<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let app = self.app;
        let title = Line::from("result ").blue();
        let instructions = Line::from(vec!["exit ".dark_gray(), "<esc>".blue(), " ".into()]);
        let block = Block::bordered()
            .title(title)
            .title_bottom(instructions)
            .padding(Padding::vertical(2))
            .borders(Borders::TOP | Borders::BOTTOM)
            .border_style(Style::new().dark_gray());

        let rows = [
            Row::new([Cell::from("wpm"), Cell::from(format!("{:.2}", app.wpm))]),
            Row::new([
                Cell::from("words"),
                Cell::from(app.words_input.len().to_string()),
            ]),
            Row::new([
                Cell::from("elapsed (s)"),
                Cell::from(format!("{:.2}", app.elapsed.as_secs_f64())),
            ]),
            Row::new([
                Cell::from("accuracy"),
                Cell::from(format!("{:.2}%", app.accuracy * 100.0)),
            ]),
        ];
        let widths = [Constraint::Percentage(50), Constraint::Percentage(50)];
        let table = Table::new(rows, widths);

        table.block(block).render(area, buf);
    }
}
