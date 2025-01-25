use crate::data_provider::{Data, Quote};
use std::cell::RefCell;
use std::rc::Rc;

pub struct AppState {
    pub mode: Rc<RefCell<Mode>>,
    pub wpm: Rc<RefCell<u16>>,
    pub screen: Rc<RefCell<Screen>>,
    pub font_size: Rc<RefCell<f32>>,
}

impl AppState {
    pub fn new(data: &Data, font_size: f32) -> Self {
        AppState {
            mode: Rc::new(RefCell::new(Mode::from_quote(
                data.get_random_quote().clone(),
            ))),
            wpm: Rc::new(RefCell::new(0)),
            screen: Rc::new(RefCell::new(Screen::TypingTest)),
            font_size: Rc::new(RefCell::new(font_size)),
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

pub fn reducer(state: &AppState, action: AppAction) {
    match action {
        AppAction::WpmChange(n) => {
            *state.wpm.borrow_mut() = n;
        }
        AppAction::FontChange(f) => {
            *state.font_size.borrow_mut() = f;
        }
        AppAction::ScreenChange(s) => {
            *state.screen.borrow_mut() = s;
        }
        AppAction::ModeChange(mode) => *state.mode.borrow_mut() = mode,
        AppAction::ModeNext(data) => (*state.mode.borrow_mut()).next(data),
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
