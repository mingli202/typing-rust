use crate::data_provider::{Data, Quote};
use std::cell::RefCell;
use std::rc::Rc;

pub struct AppState {
    pub mode: Mode,
    pub wpm: u16,
    pub screen: Screen,
    pub font_size: f32,
}

impl AppState {
    pub fn new(data: &Data, font_size: f32) -> Self {
        AppState {
            mode: Mode::from_quote(data.get_random_quote().clone()),
            wpm: 0,
            screen: Screen::TypingTest,
            font_size,
        }
    }
}

pub enum AppAction<'a> {
    FontChange(f32),
    ScreenChange(Screen),
    WpmChange(u16),
    ModeChange(Mode),
    ModeNext(&'a Data),
}

pub fn reducer(state: Rc<RefCell<AppState>>, action: AppAction) {
    match action {
        AppAction::WpmChange(n) => {
            let mut _state = state.borrow_mut();
            _state.wpm = n;
        }
        AppAction::FontChange(f) => {
            let mut _state = state.borrow_mut();
            _state.font_size = f;
        }
        AppAction::ScreenChange(s) => {
            let mut _state = state.borrow_mut();
            _state.screen = s;
        }
        AppAction::ModeChange(mode) => state.borrow_mut().mode = mode,
        AppAction::ModeNext(data) => state.borrow_mut().mode.next(data),
    }
}

#[derive(Default)]
pub enum Screen {
    #[default]
    TypingTest,
    End,
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
