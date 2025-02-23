use crate::data_provider::{Data, Quote};
use crate::Config;
use macroquad::color::Color;
use std::cell::RefCell;
use std::error::Error;
use std::fmt::Display;
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

pub struct App {
    style: Style,
    data: Data,
    config: Config,
    state: AppState,
}

impl App {
    pub fn new(data: Data, config: Config) -> Self {
        App {
            data,
            style: Style {
                font_size: Rc::new(RefCell::new(config.font_size)),
                theme: Theme::get_theme(&config.theme),
                ..Style::default()
            },
            config,
            state: AppState::default(),
        }
    }

    pub async fn main_loop(&mut self) -> Result<(), Box<dyn Error>> {
        self.state.mode = Mode::new(&self.data);

        loop {
            match self.state.screen {
                Screen::TypingTest => typing_test::run(self).await,
                Screen::End => endscreen::run(self).await,
                Screen::ThemeSelect => theme_select::run(self).await,
            };
        }
    }
}

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
enum Screen {
    TypingTest,
    End,
    ThemeSelect,
}

pub struct AppState {
    wpm: u16,
    mode: Mode,
    screen: Screen,
}

impl Default for AppState {
    fn default() -> Self {
        AppState {
            wpm: 0,
            mode: Mode::Words {
                n: 0,
                s: "".to_string(),
            },
            screen: Screen::TypingTest,
        }
    }
}

#[derive(Clone, Debug)]
pub enum Mode {
    Words { n: usize, s: String },
    Quote(Quote),
}

impl Mode {
    pub fn new(data: &Data) -> Self {
        Mode::Quote(data.get_random_quote().clone())
    }

    pub fn get_inner(&self) -> String {
        match self {
            Mode::Words { s, .. } => s.to_string(),
            Mode::Quote(q) => q.quote.clone(),
        }
    }

    pub fn next(&mut self, data: &Data) {
        let new_mode = match self {
            Mode::Words { n, .. } => Mode::Words {
                s: data
                    .get_n_random_words(*n)
                    .iter()
                    .map(|s| &(*s)[..])
                    .collect::<Vec<&str>>()
                    .join(" "),
                n: *n,
            },
            Mode::Quote(_) => Mode::Quote(data.get_random_quote().clone()),
        };
        *self = new_mode;
    }
}

impl Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Mode::Words { n, .. } => write!(f, "{} {}", n, if *n != 1 { "Words" } else { "Word" }),
            Mode::Quote(Quote { source, .. }) => write!(f, "{}", source),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Word {
    pub letters: Vec<Letter>,
    pub is_error: bool,
    pub id: usize,
    pub word: String,
    pub last_typed: usize,
}

impl Word {
    pub fn from_str(style: &Style, text: &str, id: usize) -> Word {
        Word {
            letters: text
                .chars()
                .enumerate()
                .map(|(i, letter)| Letter {
                    letter,
                    color: Rc::clone(&style.theme.ghost),
                    char_id: i,
                    word_id: id,
                })
                .collect(),
            is_error: false,
            id,
            word: text.to_string(),
            last_typed: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Letter {
    pub letter: char,
    pub color: Rc<RefCell<Color>>,
    pub char_id: usize,
    pub word_id: usize,
}

pub enum Value<T> {
    Relative(Box<dyn Fn(&Style) -> T>),
    Absolute(T),
}

impl<T: Clone> Value<T> {
    pub fn get(&self, style: &Style) -> T {
        match self {
            Self::Absolute(v) => v.clone(),
            Self::Relative(v) => v(style),
        }
    }
}

impl<T: Default> Default for Value<T> {
    fn default() -> Self {
        Value::Absolute(T::default())
    }
}
