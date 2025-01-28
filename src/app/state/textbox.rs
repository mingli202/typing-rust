use macroquad::color::Color;

use crate::app::Letter;
use std::cell::RefCell;
use std::rc::Rc;
use std::time::Instant;

pub struct TypingboxState {
    pub letters: Rc<RefCell<Vec<Letter>>>,
    pub index: Rc<RefCell<usize>>,
    pub time_started: Rc<RefCell<Instant>>,
    pub started: Rc<RefCell<bool>>,
    pub incremental_wpm: Rc<RefCell<Vec<u16>>>,
    pub timer: Rc<RefCell<Instant>>,
    pub scroll: Rc<RefCell<f32>>,
}

pub enum TypingboxAction {
    Refresh(String, Rc<RefCell<Color>>),
    TypeChar(char, Rc<RefCell<Color>>, Rc<RefCell<Color>>),
    DeleteChar(Rc<RefCell<Color>>),
    Scroll(f32),
    AddWmp(u16),
    TimerStart,
}

impl TypingboxState {
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

        TypingboxState {
            scroll: Rc::new(RefCell::new(0.0)),
            letters: Rc::new(RefCell::new(letters)),
            index: Rc::new(RefCell::new(0)),
            time_started: Rc::new(RefCell::new(Instant::now())),
            started: Rc::new(RefCell::new(false)),
            incremental_wpm: Rc::new(RefCell::new(vec![])),
            timer: Rc::new(RefCell::new(Instant::now())),
        }
    }
}

pub fn reducer(state: &TypingboxState, action: TypingboxAction) {
    match action {
        TypingboxAction::Refresh(text, ghost) => {
            let letters: Vec<Letter> = text
                .chars()
                .enumerate()
                .map(|(id, c)| Letter {
                    letter: c,
                    color: Rc::clone(&ghost),
                    id,
                })
                .collect();

            *state.letters.borrow_mut() = letters;
            *state.index.borrow_mut() = 0;
            *state.started.borrow_mut() = false;
            *state.incremental_wpm.borrow_mut() = vec![];

            *state.scroll.borrow_mut() = 0.0;
        }
        TypingboxAction::TypeChar(c, text, error) => {
            let index = *state.index.borrow();
            let letter = state.letters.borrow()[index].letter;

            state.letters.borrow_mut()[index].color = if c == letter { text } else { error };

            *state.index.borrow_mut() += 1;
        }
        TypingboxAction::DeleteChar(ghost) => {
            *state.index.borrow_mut() -= 1;

            let index = *state.index.borrow();

            state.letters.borrow_mut()[index].color = ghost;
        }
        TypingboxAction::Scroll(scroll) => *state.scroll.borrow_mut() = scroll,
        TypingboxAction::AddWmp(wpm) => {
            state.incremental_wpm.borrow_mut().push(wpm);
            *state.timer.borrow_mut() = Instant::now();
        }
        TypingboxAction::TimerStart => {
            *state.started.borrow_mut() = true;
            *state.time_started.borrow_mut() = Instant::now();
            *state.timer.borrow_mut() = Instant::now();
        }
    }
}
