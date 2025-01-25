use std::cell::RefCell;
use std::rc::Rc;

use crate::app::focus::Focus;

#[derive(Default)]
pub struct TypingtestState {
    pub focus: Rc<RefCell<TypingTestFocus>>,
}

pub enum TypingtestAction {
    FocusChange(TypingTestFocus),
    FocusNext,
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

pub fn reducer(state: &TypingtestState, action: TypingtestAction) {
    match action {
        //TypingtestAction::Click(typingbox_state, app_state, data, ghost) => {
        //    input::clear_input_queue();
        //    let focus = &state.borrow().focus;
        //    match focus {
        //        NextButton => {
        //            app_state.dispatch(AppAction::ModeNext(data));
        //
        //            let app_state = &app_state.sub();
        //            let mode = &app_state.borrow().mode;
        //
        //            typingbox_state
        //                .dispatch(TextBoxAction::Refresh(mode.get_text().to_string(), ghost));
        //
        //            state.borrow_mut().focus = Nothing;
        //        }
        //        RestartButton => {
        //            let app_state = &app_state.sub();
        //            let mode = &app_state.borrow().mode;
        //
        //            typingbox_state
        //                .dispatch(TextBoxAction::Refresh(mode.get_text().to_string(), ghost));
        //
        //            state.borrow_mut().focus = Nothing;
        //        }
        //        ThemeButton => {
        //            app_state.dispatch(AppAction::ScreenChange(Screen::ThemeSelect));
        //        }
        //        _ => (),
        //    }
        //}
        TypingtestAction::FocusChange(focus) => *state.focus.borrow_mut() = focus,
        TypingtestAction::FocusNext => state.focus.borrow_mut().next(),
    }
}
