use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Padding, Paragraph},
    Frame,
};

use crate::app::{App, LetterKind};

pub struct Ui<'a> {
    app: &'a App,
}

impl<'a> Ui<'a> {
    pub fn new(app: &'a App) -> Self {
        Self { app }
    }

    pub fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self.title(), frame.area());
        let area = Rect::new(0, 4, frame.area().width, 1);
        frame.render_widget(self.typing_area(), area);
    }

    fn title(&self) -> Line<'_> {
        Line::from("TTYPE")
    }

    fn typing_area(&self) -> Paragraph<'_> {
        let block = Block::new().padding(Padding::vertical(2));
        let text = Text::from(Line::from_iter(self.app.letters.iter().enumerate().map(
            |(i, l)| {
                Span::styled(
                    l.char.to_string(),
                    match l.kind {
                        LetterKind::Correct => LETTER_STYLE_CORRECT,
                        LetterKind::Incorrect => LETTER_STYLE_INCORRECT,
                        LetterKind::Excess => LETTER_STYLE_EXCESS,
                        LetterKind::Unreached => LETTER_STYLE_UNREACHED,
                    },
                )
                .patch_style(Style::new().bg(if i == self.app.current {
                    Color::DarkGray
                } else {
                    Color::default()
                }))
            },
        )));

        Paragraph::new(text).block(block)
    }
}

const LETTER_STYLE_CORRECT: Style = Style::new().white();
const LETTER_STYLE_INCORRECT: Style = Style::new().red();
const LETTER_STYLE_EXCESS: Style = Style::new().red().underlined();
const LETTER_STYLE_UNREACHED: Style = Style::new().dark_gray();
