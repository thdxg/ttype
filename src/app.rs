mod ui;

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use std::time::{self, Duration};

pub struct App {
    words_original: Vec<String>,
    words_input: Vec<String>,
    ctx: AppContext,
    start: Option<time::Instant>,
    end: Option<time::Instant>,
}

impl App {
    pub fn new(original: String) -> Self {
        let words_original: Vec<String> = original.split_whitespace().map(String::from).collect();
        let words_input: Vec<String> = Vec::new();
        Self {
            words_original,
            words_input,
            ctx: AppContext::Game,
            start: None,
            end: None,
        }
    }

    pub fn run(&mut self) -> Result<()> {
        ratatui::run(|terminal| {
            while self.ctx != AppContext::Finished {
                terminal.draw(|frame| {
                    frame.render_stateful_widget(ui::Ui::new(), frame.area(), self);
                })?;
                self.handle_event()?;
            }

            Ok(())
        })
    }

    fn handle_event(&mut self) -> Result<()> {
        if !event::poll(Duration::from_secs(0))? {
            return Ok(());
        }
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
                        AppContext::Stats => {}
                        _ => {}
                    },
                    KeyCode::Backspace => match self.ctx {
                        AppContext::Game => self.pop_char(),
                        AppContext::Stats => {}
                        _ => {}
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
                self.end = Some(time::Instant::now());
                self.ctx = AppContext::Stats;
            }
        }
    }

    fn pop_char(&mut self) {
        if let Some(word) = self.words_input.last_mut() {
            word.pop();
        }
    }
}

#[derive(PartialEq)]
enum AppContext {
    Game,
    Stats,
    Finished,
}
