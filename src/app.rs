mod ui;

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use std::{cmp, time};

#[derive(Default)]
pub struct App {
    title: &'static str,

    words_original: Vec<String>,
    words_input: Vec<String>,
    ctx: AppContext,

    start: Option<time::Instant>,
    end: Option<time::Instant>,
    elapsed: time::Duration,
    letters: Vec<Letter>,
    cursor: usize,
    progress: f32,
    wpm: f64,
    accuracy: f32,
}

#[derive(Default, PartialEq)]
enum AppContext {
    #[default]
    Game,
    Stats,
    Finished,
}

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
    char: char,
    kind: LetterKind,
}

const TITLE: &str = "ttype";

impl App {
    pub fn new(original: String) -> Self {
        let mut app = Self::default();
        app.title = TITLE;
        app.words_original = original.split_whitespace().map(String::from).collect();
        app.letters = create_diff(&app.words_input, &app.words_original);

        app
    }

    pub fn run(mut self) -> Result<()> {
        ratatui::run(|terminal| {
            while self.ctx != AppContext::Finished {
                if self.elapsed.is_zero()
                    && self.wpm == 0.0
                    && let Some(start) = self.start
                    && let Some(end) = self.end
                {
                    self.elapsed = end.duration_since(start);
                    self.wpm = self.words_input.len() as f64 / (self.elapsed.as_secs_f64() / 60.0);
                }
                terminal.draw(|frame| {
                    frame.render_widget(&self, frame.area());
                })?;
                self.handle_event()?;
            }

            Ok(())
        })
    }

    fn handle_event(&mut self) -> Result<()> {
        #[allow(clippy::single_match)]
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                match key_event.code {
                    KeyCode::Esc => match self.ctx {
                        AppContext::Game => {
                            self.end = Some(time::Instant::now());
                            self.ctx = AppContext::Stats;
                        }
                        AppContext::Stats => {
                            self.ctx = AppContext::Finished;
                        }
                        _ => {}
                    },
                    KeyCode::Enter => match self.ctx {
                        AppContext::Game => {
                            self.start = None;
                            self.words_input.clear();
                            self.update();
                        }
                        AppContext::Stats => {
                            self.ctx = AppContext::Finished;
                        }
                        _ => {}
                    },
                    KeyCode::Char(c) => match self.ctx {
                        AppContext::Game => {
                            if self.start.is_none() {
                                self.start = Some(time::Instant::now());
                            }
                            self.push_char(c);
                        }
                        _ => {}
                    },
                    KeyCode::Backspace => match self.ctx {
                        AppContext::Game => self.pop_char(),
                        _ => {}
                    },
                    _ => {}
                }
            }
            _ => {}
        };
        Ok(())
    }

    fn update(&mut self) {
        self.progress = self.words_input.len() as f32 / self.words_original.len() as f32;
        self.accuracy = calculate_accuracy(&self.words_input, &self.words_original);
        self.letters = create_diff(&self.words_input, &self.words_original);
        self.cursor = find_cursor(&self.words_input, &self.words_original);
    }

    fn push_char(&mut self, c: char) {
        if self.words_input.is_empty() {
            self.words_input.push(String::new());
        }

        if let Some(word) = self.words_input.last_mut() {
            match c {
                ' ' if !word.is_empty() => {
                    self.words_input.push(String::new());
                }
                ' ' => { /* do nothing */ }
                c => {
                    word.push(c);
                }
            }
        } else {
            unreachable!();
        }

        // exit if last words are equal
        if self.words_original.len() == self.words_input.len()
            && self.words_original.last() == self.words_input.last()
        {
            self.end = Some(time::Instant::now());
            self.ctx = AppContext::Stats;
        }

        self.update();
    }

    fn pop_char(&mut self) {
        if let Some(word) = self.words_input.last_mut() {
            if word.is_empty() {
                self.words_input.pop();
            } else {
                word.pop();
            }
        }

        self.update();
    }
}

fn create_diff(words_input: &[String], words_original: &[String]) -> Vec<Letter> {
    words_original
        .iter()
        .enumerate()
        .flat_map(|(i, chars_original)| {
            let mut word_diff: Vec<Letter> = Vec::new();
            match words_input.get(i) {
                Some(chars_input) => {
                    let chars_original: Vec<char> = chars_original.chars().collect();
                    let chars_input: Vec<char> = chars_input.chars().collect();
                    for j in 0..cmp::max(chars_original.len(), chars_input.len()) {
                        let mut letter = Letter::default();
                        match (chars_original.get(j), chars_input.get(j)) {
                            (Some(c_original), Some(c_input)) => {
                                letter.char = *c_original;
                                letter.kind = if c_original == c_input {
                                    LetterKind::Correct
                                } else {
                                    LetterKind::Incorrect
                                };
                            }
                            (Some(c_original), None) => {
                                letter.char = *c_original;
                                letter.kind = LetterKind::Unreached;
                            }
                            (None, Some(c_input)) => {
                                letter.char = *c_input;
                                letter.kind = LetterKind::Excess;
                            }
                            (None, None) => unreachable!(),
                        };
                        word_diff.push(letter);
                    }
                }
                None => {
                    word_diff = chars_original
                        .chars()
                        .map(|c| Letter {
                            char: c,
                            kind: LetterKind::Unreached,
                        })
                        .collect();
                }
            }

            word_diff.push(Letter {
                char: ' ',
                kind: LetterKind::Unreached,
            });

            word_diff
        })
        .collect()
}

fn find_cursor(words_input: &[String], words_original: &[String]) -> usize {
    let mut cursor = 0;
    for (w_input, w_original) in words_input.iter().zip(words_original.iter()) {
        cursor += cmp::max(w_input.len(), w_original.len());
    }

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

fn calculate_accuracy(words_input: &[String], words_original: &[String]) -> f32 {
    if words_input.is_empty() {
        return 0.0;
    }

    let correct = words_input.iter().enumerate().fold(0, |acc, (i, w_input)| {
        if let Some(w_original) = words_original.get(i)
            && w_input == w_original
        {
            acc + 1
        } else {
            acc
        }
    });

    (correct as f32) / (words_input.len() as f32)
}
