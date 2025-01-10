use std::cell::RefCell;
use std::rc::Rc;

use self::TypingTestFocus::*;
use crate::app::typing_test::textbox::TextBox;

use super::app::{Screen, ScreenState};

pub struct TypingtestState {
    pub state_screen: Rc<RefCell<ScreenState>>,
    pub focus: TypingTestFocus,
}

pub enum TypingtestAction<'a> {
    Click(&'a mut TextBox),
    FocusChange(TypingTestFocus),
}

#[derive(PartialEq, Default)]
pub enum TypingTestFocus {
    #[default]
    TypingBox,
    RestartButton,
    NextButton,
    ThemeButton,
    Nothing,
}

impl Focus for TypingTestFocus {
    fn next(&mut self) {
        match self {
            TypingTestFocus::Nothing => *self = TypingTestFocus::NextButton,
            TypingTestFocus::NextButton => *self = TypingTestFocus::RestartButton,
            TypingTestFocus::RestartButton => *self = TypingTestFocus::ThemeButton,
            TypingTestFocus::ThemeButton => *self = TypingTestFocus::NextButton,
            TypingTestFocus::TypingBox => *self = TypingTestFocus::NextButton,
        }
    }
}

pub fn reducer(state: Rc<RefCell<TypingtestState>>, action: TypingtestAction) {
    match action {
        TypingtestAction::Click(typingbox) => {
            input::clear_input_queue();
            let focus = state.borrow().focus;
            match focus {
                NextButton => {
                    let data = state.borrow().state_screen.borrow().data;
                    state.borrow().state_screen.borrow_mut().mode.next(&data);

                    let mode = state.borrow().state_screen.borrow().mode;
                    typingbox.refresh(mode.get_text());

                    state.borrow_mut().focus = Noting;
                }
                RestartButton => {
                    let mode = state.borrow().state_screen.borrow().mode;
                    typingbox.refresh(mode.get_text());

                    state.borrow_mut().focus = Noting;
                }
                ThemeButton => {
                    *state.borrow().state_screen.borrow_mut().screen = app::ThemeSelect;
                }
                _ => (),
            }
        }
        TypingtestAction::FocusChange(focus) => state.borrow_mut().focus = focus,
    }
}
