use crate::data_provider::Data;
use crate::Config;
use macroquad::color::Color;
use std::cell::RefCell;
use std::rc::Rc;
mod theme;

mod style;
use style::{BorderParams, Style};

use self::state::screen::{AppAction, Screen};
use self::state::State;
use self::theme::Theme;
pub use self::theme::ThemeName;

mod state;
use state::screen::{reducer, AppState};

mod text;
mod util;

// mod endscreen;
mod focus;
// mod theme_select;
mod typing_test;

pub struct App<'a> {
    pub style: Style,
    pub state: State<AppState, AppAction<'a>>,
    pub data: Data,
    pub config: Config,
}

impl<'a> App<'a> {
    pub fn new(data: Data, config: Config) -> Self {
        App {
            style: Style {
                font_size: Rc::new(RefCell::new(config.font_size)),
                theme: Theme::get_theme(&config.theme),
                ..Style::default()
            },
            state: State::new(AppState::new(&data, config.font_size), reducer),
            data,
            config,
        }
    }

    pub async fn run(&'a self) {
        loop {
            let screen = &self.state.get().screen;
            match screen {
                Screen::TypingTest => typing_test::run(self).await,
                Screen::End => (),         //endscreen::run(&state).await,
                Screen::ThemeSelect => (), // theme_select::run(&state).await,
            };
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
