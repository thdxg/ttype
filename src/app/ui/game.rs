use crate::app::{App, Letter, LetterKind};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Padding, Paragraph, Widget},
};
use std::cmp;

pub struct Game<'a> {
    app: &'a App<'a>,
}

impl<'a> Game<'a> {
    pub fn new(app: &'a App) -> Self {
        Self { app }
    }
}

impl<'a> Widget for Game<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let app = self.app;
        let title = Line::from(format!("{} ", app.title)).blue();
        let progress = Line::from(format!(" {:.1}%", app.progress * 100.0))
            .blue()
            .right_aligned();
        let instructions = Line::from(vec![
            "restart ".dark_gray(),
            "<enter> ".blue(),
            "end ".dark_gray(),
            "<esc>".blue(),
            " ".into(),
        ]);
        let block = Block::bordered()
            .title(title)
            .title(progress)
            .title_bottom(instructions)
            .padding(Padding::vertical(2))
            .borders(Borders::TOP | Borders::BOTTOM)
            .border_style(Style::new().dark_gray());

        let width = area.width as usize;
        let mut visible = 0..cmp::min(width, app.letters.len());
        if app.cursor > width / 2 {
            visible.start = cmp::max(0, app.cursor - width / 2);
            visible.end = cmp::min(app.letters.len(), app.cursor + width / 2);
        }

        let body = Paragraph::new(Text::from(Line::from(
            app.letters[visible.clone()]
                .iter()
                .enumerate()
                .map(|(i, letter)| {
                    Span::from(letter.char.to_string()).style(letter.style().bg(
                        if visible.start + i == app.cursor {
                            Color::DarkGray
                        } else {
                            Color::default()
                        },
                    ))
                })
                .collect::<Vec<Span>>(),
        )));

        body.block(block).render(area, buf);
    }
}

const LETTER_STYLE_CORRECT: Style = Style::new().white();
const LETTER_STYLE_INCORRECT: Style = Style::new().red();
const LETTER_STYLE_EXCESS: Style = Style::new().red().underlined();
const LETTER_STYLE_UNREACHED: Style = Style::new().dark_gray();

impl Letter {
    fn style(&self) -> Style {
        match self.kind {
            LetterKind::Correct => LETTER_STYLE_CORRECT,
            LetterKind::Incorrect => LETTER_STYLE_INCORRECT,
            LetterKind::Excess => LETTER_STYLE_EXCESS,
            LetterKind::Unreached => LETTER_STYLE_UNREACHED,
        }
    }
}
