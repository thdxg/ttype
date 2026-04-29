use std::time;

use crate::app::App;
use anyhow::{Error, Result};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Rect},
    widgets::{Cell, Row, Table, Widget},
    DefaultTerminal,
};

pub fn draw(app: &App, terminal: &mut DefaultTerminal) -> Result<()> {
    let stats = Stats::new(app)?;
    terminal.draw(|frame| {
        frame.render_widget(stats, frame.area());
    })?;
    Ok(())
}

struct Stats {
    wpm: f64,
    words: usize,
    start: time::Instant,
    end: time::Instant,
    elapsed: time::Duration,
}

impl Stats {
    fn new(app: &App) -> Result<Self> {
        let Some(start) = app.start else {
            return Err(Error::msg("game not started"));
        };
        let Some(end) = app.start else {
            return Err(Error::msg("game not stopped"));
        };
        let words = app.words_input.len();
        let elapsed = end.duration_since(start);
        let wpm = words as f64 / (elapsed.as_secs_f64() / 60.0);
        Ok(Stats {
            wpm,
            words,
            start,
            end,
            elapsed,
        })
    }
}

impl Widget for Stats {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let header = Row::new([Cell::from("Stat"), Cell::from("Value")]);
        let rows = [
            // Row::new([Cell::from("start"), Cell::from(self.start.)]),
            Row::new([Cell::from("end"), Cell::from(self.wpm.to_string())]),
            Row::new([Cell::from("wpm"), Cell::from(self.wpm.to_string())]),
            Row::new([Cell::from("words"), Cell::from(self.words.to_string())]),
            Row::new([
                Cell::from("elapsed"),
                Cell::from(self.elapsed.as_secs_f64().to_string()),
            ]),
        ];
        let widths = [Constraint::Percentage(50), Constraint::Percentage(50)];
        let table = Table::new(rows, widths).header(header);

        table.render(area, buf);
    }
}
