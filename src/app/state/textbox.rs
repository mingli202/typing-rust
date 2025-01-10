use macroquad::color::Color;

use crate::app::Letter;
use std::cell::RefCell;
use std::rc::Rc;
use std::time::Instant;

pub struct TextBoxState {
    pub letters: Vec<Letter>,
    pub index: usize,
    pub time_started: Instant,
    pub started: bool,
    pub incremental_wpm: Vec<u16>,
    pub timer: Instant,
    pub scroll: f32,
}

pub enum TextBoxAction {
    Refresh(String, Rc<RefCell<Color>>),
    TypeChar(char, Rc<RefCell<Color>>, Rc<RefCell<Color>>),
    DeleteChar(Rc<RefCell<Color>>),
    Scroll(f32),
    AddWmp(u16),
    TimerStart,
}

impl TextBoxState {
    pub fn new(text: &str, ghost: Rc<RefCell<Color>>) -> Self {
        let letters: Vec<Letter> = text
            .chars()
            .enumerate()
            .map(|(id, c)| Letter {
                letter: c,
                color: Rc::clone(&ghost),
                id,
            })
            .collect();

        TextBoxState {
            scroll: 0.0,
            letters,
            index: 0,
            time_started: Instant::now(),
            started: false,
            incremental_wpm: vec![],
            timer: Instant::now(),
        }
    }
}

pub fn reducer(state: Rc<RefCell<TextBoxState>>, action: TextBoxAction) {
    match action {
        TextBoxAction::Refresh(text, ghost) => {
            let letters: Vec<Letter> = text
                .chars()
                .enumerate()
                .map(|(id, c)| Letter {
                    letter: c,
                    color: Rc::clone(&ghost),
                    id,
                })
                .collect();

            let mut state = state.borrow_mut();
            state.letters = letters;
            state.index = 0;
            state.started = false;
            state.incremental_wpm = vec![];

            state.scroll = 0.0;
        }
        TextBoxAction::TypeChar(c, text, error) => {
            let letters = &state.borrow().letters;
            let index = state.borrow().index;

            let updated_letter = if c == letters[index].letter {
                Letter {
                    color: text,
                    letter: letters[index].letter,
                    id: letters[index].id,
                }
            } else {
                Letter {
                    color: error,
                    letter: letters[index].letter,
                    id: letters[index].id,
                }
            };

            let mut state = state.borrow_mut();
            state.letters[index] = updated_letter;

            state.index += 1;
        }
        TextBoxAction::DeleteChar(ghost) => {
            let mut state = state.borrow_mut();
            state.index -= 1;

            let index = state.index;

            state.letters[index] = Letter {
                color: ghost,
                ..state.letters[state.index]
            };
        }
        TextBoxAction::Scroll(scroll) => state.borrow_mut().scroll = scroll,
        TextBoxAction::AddWmp(wpm) => {
            let mut state = state.borrow_mut();
            state.incremental_wpm.push(wpm);
            state.timer = Instant::now();
        }
        TextBoxAction::TimerStart => {
            let mut state = state.borrow_mut();
            state.started = true;
            state.time_started = Instant::now();
            state.timer = Instant::now();
        }
    }
}
