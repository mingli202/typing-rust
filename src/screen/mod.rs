use crate::data_provider::Data;
use macroquad::color::Color;
use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;
mod component;
mod theme;

use component::Style;
mod text;
mod typing_test;
mod util;

pub enum Mode {
    WordCount(usize),
    TimeSec(usize),
    Quote,
}

impl Default for Mode {
    fn default() -> Self {
        Mode::TimeSec(30)
    }
}

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
    // buttons: HashMap<State, Vec<Box<dyn Component>>>,
    // focus: Rc<RefCell<i32>>,
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
    //let mut next_button =
    //    next_button::NextButton::new(&scr.style, Rc::clone(&scr.focus), Rc::clone(&typingbox));
    //let mut quit_button = quit_button::QuitButton::new(&scr.style, Rc::clone(&scr.focus));

    loop {
        (*scr).state = match scr.state {
            State::TypingTest => typing_test::run(scr).await,
            State::EndScreen => State::EndScreen,
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
