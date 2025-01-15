use crate::data_provider::Data;
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
enum State {
    TypingTest,
    EndScreen,
    ThemeSelect,
}

pub struct Screen {
    style: Style,
    state: State,
    data: Data,
    config: Config,
}

impl Screen {
    pub fn new(data: Data, config: Config) -> Self {
        Screen {
            data,
            state: State::TypingTest,
            style: Style {
                font_size: Rc::new(RefCell::new(config.font_size)),
                theme: Theme::get_theme(&config.theme),
                ..Style::default()
            },
            config,
        }
    }
}

pub async fn main_loop(scr: &mut Screen) -> Result<(), Box<dyn Error>> {
    let mut wpm = 0;
    let mut text = scr.data.get_random_quote().quote.clone(); // TODO: remove clone

    loop {
        scr.state = match scr.state {
            State::TypingTest => typing_test::run(scr, &mut wpm, &mut text).await,
            State::EndScreen => endscreen::run(scr, &wpm, &mut text).await,
            State::ThemeSelect => theme_select::run(scr).await,
        };
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
