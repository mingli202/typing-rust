use std::cell::RefCell;
use std::rc::Rc;

use crate::app::style::Style;
use crate::app::theme::Theme;
use crate::data_provider::{Data, Quote};
use crate::Config;

pub struct ScreenState {
    pub mode: Mode,
    pub wpm: u16,
    pub screen: Screen,
    pub style: Style,
    pub data: Data,
    pub config: Config,
}

impl ScreenState {
    pub fn new(data: Data, config: Config) -> Self {
        ScreenState {
            mode: Mode::from_quote(data.get_random_quote().clone()),
            wpm: 0,
            style: Style {
                font_size: Rc::new(RefCell::new(config.font_size)),
                theme: Theme::get_theme(&config.theme),
                ..Style::default()
            },
            config,
            data,
            screen: app::TypingTest,
        }
    }
}

pub enum ScreenAction {
    FontChange(f32),
    ScreenChange(Screen),
    WpmChange(u16),
}

pub fn reducer(state: Rc<RefCell<ScreenState>>, action: ScreenAction) {
    match action {
        ScreenAction::WpmChange(n) => {
            let mut _state = state.borrow_mut();
            _state.wpm = n;
        }
        ScreenAction::FontChange(f) => {
            let _state = state.borrow_mut();
            *_state.style.font_size.borrow_mut() = f;
        }
        ScreenAction::ScreenChange(s) => {
            let mut _state = state.borrow_mut();
            _state.screen = s;
        }
    }
}

#[derive(Default)]
pub enum Screen {
    #[default]
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
