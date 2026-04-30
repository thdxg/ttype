use crate::app::App;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Padding, Paragraph, Widget},
};
use std::cmp;

pub struct Game {
    title: String,
    letters: Vec<Letter>,
    cursor: usize,
    progress: f32,
}

impl Game {
    pub fn new(app: &App) -> Self {
        let mut letters = create_diff(&app.words_input, &app.words_original);
        let cursor = find_cursor(&app.words_input, &app.words_original);
        letters[cursor].current = true;
        let progress = app.words_input.len() as f32 / app.words_original.len() as f32;
        Game {
            title: "ttype".into(),
            letters,
            cursor,
            progress,
        }
    }
}

impl Widget for Game {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(format!("{} ", self.title)).blue();
        let progress = Line::from(format!(" {:.1}%", self.progress * 100.0))
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
        let mut visible = 0..cmp::min(width, self.letters.len());
        if self.cursor > width / 2 {
            visible.start = cmp::max(0, self.cursor - width / 2);
            visible.end = cmp::min(self.letters.len(), self.cursor + width / 2);
        }

        let body = Paragraph::new(Text::from(Line::from(
            self.letters[visible]
                .iter()
                .map(|l| Span::from(l.char.to_string()).style(l.style()))
                .collect::<Vec<Span>>(),
        )));

        body.block(block).render(area, buf);
    }
}

fn create_diff(words_input: &[String], words_original: &[String]) -> Vec<Letter> {
    words_original
        .iter()
        .enumerate()
        .flat_map(|(i, w_original)| {
            let mut word_diff: Vec<Letter> = Vec::new();
            match words_input.get(i) {
                Some(w_input) => {
                    for j in 0..cmp::max(w_original.len(), w_input.len()) {
                        let mut letter = Letter::default();
                        match (w_original.chars().nth(j), w_input.chars().nth(j)) {
                            (Some(c_original), Some(c_input)) => {
                                letter.char = c_original;
                                letter.kind = if c_original == c_input {
                                    LetterKind::Correct
                                } else {
                                    LetterKind::Incorrect
                                };
                            }
                            (Some(c_original), None) => {
                                letter.char = c_original;
                                letter.kind = LetterKind::Unreached;
                            }
                            (None, Some(c_input)) => {
                                letter.char = c_input;
                                letter.kind = LetterKind::Excess;
                            }
                            (None, None) => unreachable!(),
                        };
                        word_diff.push(letter);
                    }
                }
                None => {
                    word_diff = w_original
                        .chars()
                        .map(|c| Letter {
                            char: c,
                            kind: LetterKind::Unreached,
                            current: false,
                        })
                        .collect();
                }
            }

            word_diff.push(Letter {
                char: ' ',
                kind: LetterKind::Unreached,
                current: false,
            });

            word_diff
        })
        .collect()
}

fn find_cursor(words_input: &[String], words_original: &[String]) -> usize {
    let mut cursor = 0;
    words_input
        .iter()
        .zip(words_original.iter())
        .for_each(|(w_input, w_original)| {
            cursor += cmp::max(w_input.len(), w_original.len());
        });

    if !words_input.is_empty() {
        cursor += words_input.len() - 1; // account for spaces
    }

    if !words_input.is_empty()
        && let Some(w_last_original) = words_original.get(words_input.len() - 1)
        && let Some(w_last_input) = words_input.last()
    {
        let offset = w_last_original.len() as isize - w_last_input.len() as isize;
        if offset > 0 {
            cursor -= offset as usize;
        }
    }

    cursor
}

const LETTER_STYLE_CORRECT: Style = Style::new().white();
const LETTER_STYLE_INCORRECT: Style = Style::new().red();
const LETTER_STYLE_EXCESS: Style = Style::new().red().underlined();
const LETTER_STYLE_UNREACHED: Style = Style::new().dark_gray();

#[derive(Debug, PartialEq, Default)]
enum LetterKind {
    Correct,
    Incorrect,
    Excess,
    #[default]
    Unreached,
}

#[derive(Debug, Default)]
struct Letter {
    pub char: char,
    pub kind: LetterKind,
    pub current: bool,
}

impl Letter {
    fn style(&self) -> Style {
        match self.kind {
            LetterKind::Correct => LETTER_STYLE_CORRECT,
            LetterKind::Incorrect => LETTER_STYLE_INCORRECT,
            LetterKind::Excess => LETTER_STYLE_EXCESS,
            LetterKind::Unreached => LETTER_STYLE_UNREACHED,
        }
        .patch(if self.current {
            Style::new().bg(Color::DarkGray)
        } else {
            Style::new().bg(Color::default())
        })
    }
}
