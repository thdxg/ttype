mod ui;

use anyhow::{Context, Result};
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::DefaultTerminal;

pub struct App {
    original: String,
    original_words: Vec<String>,
    letters: Vec<Letter>,
    current: usize,
    exit: bool,
}

impl App {
    pub fn new(original: String) -> Self {
        let original_words: Vec<String> = original.split_whitespace().map(String::from).collect();
        let letters = original
            .chars()
            .map(|c| Letter {
                char: c,
                kind: LetterKind::Unreached,
            })
            .collect();
        Self {
            original,
            original_words,
            letters,
            current: 0,
            exit: false,
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while !self.exit {
            terminal.draw(|frame| ui::Ui::new(self).draw(frame))?;
            self.handle_event()?;
        }
        Ok(())
    }

    fn handle_event(&mut self) -> Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                match key_event.code {
                    KeyCode::Esc => self.exit = true,
                    KeyCode::Enter => todo!("reset"),
                    KeyCode::Char(c) => {
                        self.push_letter(c)?;
                    }
                    KeyCode::Backspace => {
                        self.pop_letter()?;
                    }
                    _ => {}
                }
            }
            _ => {}
        };
        Ok(())
    }

    fn push_letter(&mut self, char_input: char) -> Result<()> {
        let (word_idx, char_idx) =
            self.letters
                .iter()
                .take(self.current)
                .fold((0, 0), |(wi, ci), l| {
                    if l.char == ' ' {
                        (wi + 1, 0)
                    } else if !char_input.is_whitespace() {
                        (wi, ci + 1)
                    } else {
                        (wi, ci)
                    }
                });

        let word = self
            .original_words
            .get(word_idx)
            .context("input words exceeded original words")?;

        if let Some(char_original) = word.chars().nth(char_idx) {
            if char_original == char_input {
                // correct character
                // TODO: better out-of-bounds handling
                let letter = self.letters.get_mut(self.current).unwrap();
                letter.kind = LetterKind::Correct;
            } else {
                // incorrect character
                // TODO: better out-of-bounds handling
                let letter = self.letters.get_mut(self.current).unwrap();
                letter.kind = LetterKind::Incorrect;
            }
        } else {
            // excess character
            self.letters.insert(
                self.current,
                Letter {
                    char: char_input,
                    kind: LetterKind::Excess,
                },
            );
        }

        self.current += 1;

        Ok(())
    }

    fn pop_letter(&mut self) -> Result<()> {
        if self.current == 0 {
            return Ok(());
        }

        let letter = self.letters.get_mut(self.current - 1).unwrap();
        if letter.kind == LetterKind::Excess {
            self.letters.remove(self.current - 1);
        } else {
            letter.kind = LetterKind::Unreached;
        }

        self.current -= 1;

        Ok(())
    }
}

#[derive(Debug, PartialEq)]
pub enum LetterKind {
    Correct,
    Incorrect,
    Excess,
    Unreached,
}

#[derive(Debug)]
pub struct Letter {
    char: char,
    kind: LetterKind,
}
