use std::time;

use crate::app::App;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Rect},
    style::{Style, Stylize},
    text::Line,
    widgets::{Block, Borders, Cell, Padding, Row, Table, Widget},
};

#[derive(Default)]
pub struct Stats {
    wpm: f64,
    words: usize,
    elapsed: time::Duration,
    accuracy: f32,
}

impl Stats {
    pub fn new(app: &App) -> Self {
        let Some(start) = app.start else {
            return Self::default();
        };
        let end = app.end.unwrap();
        let words = app.words_input.len();
        let elapsed = end.duration_since(start);
        let wpm = words as f64 / (elapsed.as_secs_f64() / 60.0);
        let (total, correct) =
            app.words_input
                .iter()
                .enumerate()
                .fold((0, 0), |(tot, cor), (i, w_input)| {
                    if let Some(w_original) = app.words_original.get(i) {
                        if w_input == w_original {
                            (tot + 1, cor + 1)
                        } else {
                            (tot + 1, cor)
                        }
                    } else {
                        (tot, cor)
                    }
                });
        let accuracy = (correct as f32) / (total as f32);
        Stats {
            wpm,
            words,
            elapsed,
            accuracy,
        }
    }
}

impl Widget for Stats {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from("result ").blue();
        let instructions = Line::from(vec!["exit ".dark_gray(), "<esc>".blue(), " ".into()]);
        let block = Block::bordered()
            .title(title)
            .title_bottom(instructions)
            .padding(Padding::vertical(2))
            .borders(Borders::TOP | Borders::BOTTOM)
            .border_style(Style::new().dark_gray());

        let rows = [
            Row::new([
                Cell::from("wpm"),
                Cell::from(format!("{:.2}", self.wpm.to_string())),
            ]),
            Row::new([Cell::from("words"), Cell::from(self.words.to_string())]),
            Row::new([
                Cell::from("elapsed (s)"),
                Cell::from(format!("{:.2}", self.elapsed.as_secs_f64().to_string())),
            ]),
            Row::new([
                Cell::from("accuracy"),
                Cell::from(format!("{:.2}%", self.accuracy * 100.0)),
            ]),
        ];
        let widths = [Constraint::Percentage(50), Constraint::Percentage(50)];
        let table = Table::new(rows, widths);

        table.block(block).render(area, buf);
    }
}
