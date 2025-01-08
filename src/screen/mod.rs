use crate::data_provider::{Data, Quote};
use crate::Config;
use macroquad::color::Color;
use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;
mod theme;

mod style;
use style::{BorderParams, Style};

use self::theme::Theme;
pub use self::theme::ThemeName;

mod text;
mod util;

mod endscreen;
mod focus;
mod theme_select;
mod typing_test;

pub struct Screen {
    style: Style,
    data: Data,
    config: Config,
}

impl Screen {
    pub fn new(data: Data, config: Config) -> Self {
        Screen {
            data,
            style: Style {
                font_size: Rc::new(RefCell::new(config.font_size)),
                theme: Theme::get_theme(&config.theme),
                ..Style::default()
            },
            config,
        }

        // TODO: animation library use threads to mutate value over time (maybe)
    }
}

type ReturnType<'a> = (State, u16, Mode<'a>);

pub async fn main_loop(scr: &mut Screen) -> Result<(), Box<dyn Error>> {
    let mut wpm = 0;

    let quote = scr.data.get_random_quote();
    let mut mode = Mode::from_quote(quote);
    let mut state = State::TypingTest;

    loop {
        (state, wpm, mode) = match state {
            State::TypingTest => typing_test::run(scr, wpm, mode).await,
            State::EndScreen => endscreen::run(scr, wpm, mode).await,
            State::ThemeSelect => theme_select::run(scr).await,
        };
    }
}

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
enum State {
    TypingTest,
    EndScreen,
    ThemeSelect,
}

enum Mode<'a> {
    Quote(&'a Quote),
    Words(String),
}

impl<'a> Mode<'a> {
    pub fn from_quote(quote: &'a Quote) -> Self {
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

    pub fn next(&mut self, data: &'a Data) {
        *self = match self {
            Mode::Words(words) => Mode::from_words(data.get_n_random_words(words.len())),
            Mode::Quote(_) => Mode::from_quote(data.get_random_quote()),
        }
    }
}

pub struct Word {
    pub word: Vec<Letter>,
    pub index: usize,
    pub is_correct: bool,
}

impl Word {
    pub fn new(word: Vec<Letter>, index: usize) -> Word {
        Word {
            word,
            index,
            is_correct: true,
        }
    }

    pub fn from_str(word: &str, ghost: Rc<RefCell<Color>>, index: usize) -> Self {
        Word {
            word: word
                .chars()
                .map(|c| Letter {
                    letter: c,
                    color: Rc::clone(&ghost),
                    id: 0,
                })
                .collect(),
            is_correct: true,
            index,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Letter {
    pub letter: char,
    pub color: Rc<RefCell<Color>>,
    pub id: usize,
}

pub enum Value<T> {
    Relative(Box<dyn Fn() -> T + 'static>),
    Absolute(T),
}

impl<T: Clone> Value<T> {
    pub fn get(&self) -> T {
        match self {
            Self::Absolute(v) => v.clone(),
            Self::Relative(v) => v(),
        }
    }
}
