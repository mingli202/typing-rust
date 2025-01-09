use std::cell::RefCell;
use std::rc::Rc;

use crate::data_provider::{Data, Quote};
use crate::Config;

use super::style::Style;
use super::theme::Theme;

pub struct State {
    pub mode: Rc<RefCell<Mode>>,
    pub wpm: Rc<RefCell<usize>>,
    pub screen: Rc<RefCell<Screen>>,
    pub style: Style,
    pub data: Data,
    pub config: Config,
}

impl State {
    pub fn new(data: Data, config: Config) -> Self {
        State {
            mode: Rc::new(RefCell::new(Mode::from_quote(
                data.get_random_quote().clone(),
            ))),
            wpm: Rc::new(RefCell::new(0)),
            style: Style {
                font_size: Rc::new(RefCell::new(config.font_size)),
                theme: Theme::get_theme(&config.theme),
                ..Style::default()
            },
            config,
            data,
            screen: Rc::new(RefCell::new(Screen::TypingTest)),
        }
    }

    pub fn dispatch(&self, action: Action) {
        match action {
            Action::FontChange(n) => *self.style.font_size.borrow_mut() += n,
        }
    }
}

pub enum Action {
    FontChange(f32),
}

pub enum Screen {
    TypingTest,
    EndScreen,
    ThemeSelect,
}

pub enum Mode {
    Quote(Quote),
    Words(String),
}

impl Mode {
    pub fn from_quote(quote: Quote) -> Self {
        Mode::Quote(quote)
    }

    pub fn from_words(words: Vec<&String>) -> Self {
        Mode::Words(words.iter().fold(String::new(), |acc, s| acc + s))
    }

    pub fn get_text(&self) -> &str {
        match self {
            Mode::Words(words) => words,
            Mode::Quote(quote) => &quote.quote,
        }
    }

    pub fn next(&mut self, data: &Data) {
        *self = match self {
            Mode::Words(words) => Mode::from_words(data.get_n_random_words(words.len())),
            Mode::Quote(_) => Mode::from_quote(data.get_random_quote().clone()),
        }
    }
}
