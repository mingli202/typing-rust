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

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
enum Screen {
    TypingTest,
    End,
    ThemeSelect,
}

pub struct App {
    style: Style,
    data: Data,
    config: Config,
    state: AppState,
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
            mode: Mode::Text("".to_string()),
            screen: Screen::TypingTest,
        }
    }
}

pub enum Mode {
    Text(String),
    Quote(Quote),
}

impl Mode {
    pub fn get(&self) -> String {
        match self {
            Mode::Text(s) => s.to_string(),
            Mode::Quote(q) => q.quote.clone(),
        }
    }
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
        let text = self.data.get_random_quote().quote.clone();

        self.state.mode = Mode::Text(text.clone());

        loop {
            match self.state.screen {
                Screen::TypingTest => typing_test::run(self).await,
                Screen::End => endscreen::run(self).await,
                Screen::ThemeSelect => theme_select::run(self).await,
            };
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
    Relative(Box<dyn Fn() -> T>),
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
