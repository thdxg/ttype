mod ui;

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use std::time::{self, Duration};

pub struct App {
    words_original: Vec<String>,
    words_input: Vec<String>,
    start: Option<time::Instant>,
    end: Option<time::Instant>,
    exit: bool,
}

impl App {
    pub fn new(original: String) -> Self {
        let words_original: Vec<String> = original.split_whitespace().map(String::from).collect();
        let words_input: Vec<String> = Vec::new();
        Self {
            words_original,
            words_input,
            start: None,
            end: None,
            exit: false,
        }
    }

    pub fn run(&mut self) -> Result<()> {
        ratatui::run(|terminal| {
            while self.end.is_none() {
                ui::game::draw(self, terminal)?;
                self.handle_event(Context::Game)?;
            }
            while !self.exit {
                ui::stats::draw(self, terminal)?;
                self.handle_event(Context::Stats)?;
            }
            Ok(())
        })
    }

    fn handle_event(&mut self, ctx: Context) -> Result<()> {
        if !event::poll(Duration::from_secs(0))? {
            return Ok(());
        }
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                match key_event.code {
                    KeyCode::Esc => match ctx {
                        Context::Game => self.end = Some(time::Instant::now()),
                        Context::Stats => self.exit = true,
                    },
                    KeyCode::Enter => match ctx {
                        Context::Game => todo!("reset"),
                        Context::Stats => self.exit = true,
                    },
                    KeyCode::Char(c) => match ctx {
                        Context::Game => self.push_char(c),
                        Context::Stats => {}
                    },
                    KeyCode::Backspace => match ctx {
                        Context::Game => self.pop_char(),
                        Context::Stats => {}
                    },
                    _ => {}
                }
            }
            _ => {}
        };
        Ok(())
    }

    fn push_char(&mut self, c: char) {
        if self.words_input.is_empty() {
            self.start = Some(time::Instant::now());
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
        if self.words_original.len() == self.words_input.len() {
            let a = self.words_original.last().map(|w| w.len());
            let b = self.words_input.last().map(|w| w.len());
            if a == b {
                self.end = Some(time::Instant::now())
            }
        }
    }

    fn pop_char(&mut self) {
        if let Some(word) = self.words_input.last_mut() {
            word.pop();
        }
    }
}

enum Context {
    Game,
    Stats,
}
