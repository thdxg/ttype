use crate::app::App;
use anyhow::Result;
use ratatui::{
    DefaultTerminal,
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Padding, Paragraph, Widget},
};
use std::cmp;

pub fn draw(app: &App, terminal: &mut DefaultTerminal) -> Result<()> {
    let game = Game::new(app);
    terminal.draw(|frame| {
        frame.render_widget(game, frame.area());
    })?;
    Ok(())
}

struct Game {
    title: String,
    letters: Vec<Letter>,
    cursor: usize,
}

impl Game {
    fn new(app: &App) -> Self {
        let mut letters = create_diff(&app.words_input, &app.words_original);
        let cursor = find_cursor(&app.words_input, &app.words_original);
        letters[cursor].current = true;
        Game {
            title: "ttype".into(),
            letters,
            cursor,
        }
    }
}

impl Widget for Game {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(self.title);

        let width = area.width as usize;
        let mut visible = 0..cmp::min(width, self.letters.len());
        if self.cursor > width / 2 {
            visible.start = cmp::max(0, self.cursor - width / 2);
            visible.end = cmp::min(self.letters.len(), self.cursor + width / 2);
        }

        let typing_area = Paragraph::new(Text::from(Line::from(
            self.letters[visible]
                .iter()
                .map(|l| Span::from(l.char.to_string()).style(l.style()))
                .collect::<Vec<Span>>(),
        )))
        .block(Block::new().padding(Padding::uniform(2)));

        title.render(area, buf);
        typing_area.render(area, buf);
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
