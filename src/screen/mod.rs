use crate::data_provider::Data;
use macroquad::color::Color;
use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;
mod component;
mod theme;

use component::Style;
mod text;
mod util;

mod endscreen;
mod typing_test;

//pub enum Mode {
//    WordCount(usize),
//    TimeSec(usize),
//    Quote,
//}
//
//impl Default for Mode {
//    fn default() -> Self {
//        Mode::TimeSec(30)
//    }
//}

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
}

impl Screen {
    pub fn new(data: Data) -> Self {
        Screen {
            data,
            state: State::TypingTest,
            style: Style {
                font_size: 30.0,
                ..Style::default()
            },
        }

        // TODO: animation library use threads to mutate value over time (maybe)
    }
}

pub async fn main_loop(scr: &mut Screen) -> Result<(), Box<dyn Error>> {
    let mut wpm = 0;

    loop {
        scr.state = match scr.state {
            State::TypingTest => typing_test::run(scr, &mut wpm).await,
            State::EndScreen => endscreen::run(scr, &wpm).await,
            State::ThemeSelect => State::ThemeSelect,
        };
    }
}

#[derive(Debug, Clone)]
pub struct Letter {
    pub letter: char,
    pub color: Rc<RefCell<Color>>,
    pub id: usize,
}
