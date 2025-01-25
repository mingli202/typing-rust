use std::rc::Rc;

use macroquad::input::{self, KeyCode, MouseButton};
use macroquad::math::Vec2;
use macroquad::window;

use crate::app::util;

mod next_button;
mod restart_button;
pub mod textbox;
mod theme_button;
mod tracker;

use super::state::screen::{AppAction, Screen};
use super::state::textbox::TypingAction;
use super::state::typing_test::*;
use super::state::State;
use super::App;

pub async fn run<'a>(app: &'a App<'a>) {
    input::clear_input_queue();

    let state = State::new(TypingtestState::default(), reducer);

    let style = &app.style;

    let typingbox = textbox::TextBox::new(style, app.state.get().mode.borrow().get_text());

    let tracker = tracker::Tracker::new(style);
    let next_button = next_button::NextButton::new(style);
    let restart_button = restart_button::RestartButton::new(style);
    let theme_button = theme_button::ThemeButton::new(style);

    let click = || {
        input::clear_input_queue();

        let focus = state.get().focus.borrow();
        match *focus {
            TypingTestFocus::NextButton => {
                app.state.dispatch(AppAction::ModeNext(&app.data));

                typingbox.state.dispatch(TypingAction::Refresh(
                    app.state.get().mode.borrow().get_text().to_string(),
                    Rc::clone(&app.style.theme.ghost),
                ));
            }
            TypingTestFocus::RestartButton => {
                let mode = app.state.get().mode.borrow();

                typingbox.state.dispatch(TypingAction::Refresh(
                    mode.get_text().to_string(),
                    Rc::clone(&app.style.theme.ghost),
                ));
            }
            TypingTestFocus::ThemeButton => {
                app.state
                    .dispatch(AppAction::ScreenChange(Screen::ThemeSelect));
            }
            _ => (),
        }
    };

    loop {
        if let Some(k) = input::get_last_key_pressed() {
            let _ = util::handle_resize(&app.state, &app.config);

            match k {
                KeyCode::Backspace => {
                    input::clear_input_queue();
                    state.dispatch(TypingtestAction::FocusChange(TypingTestFocus::TypingBox));
                    typingbox.delete_char();
                }
                KeyCode::Enter => click(),
                KeyCode::Tab => {
                    input::clear_input_queue();
                    state.dispatch(TypingtestAction::FocusNext);
                }
                // this passes the keytrokes to type
                _ => {
                    if let Some(c) = input::get_char_pressed() {
                        state.dispatch(TypingtestAction::FocusChange(TypingTestFocus::TypingBox));

                        if typingbox.on_type(c) {
                            app.state
                                .dispatch(AppAction::WpmChange(typingbox.get_wpm(None)));
                            app.state.dispatch(AppAction::ScreenChange(Screen::End));
                        }
                    }
                }
            }
        }

        if *state.get().focus.borrow() == TypingTestFocus::TypingBox {
            input::show_mouse(false);
        } else {
            input::show_mouse(true);
        }

        if input::is_mouse_button_pressed(MouseButton::Left) {
            click();
        }

        match input::mouse_delta_position() {
            Vec2 { x: dx, y: dy } if dx != 0.0 && dy != 0.0 => {
                let f = if util::is_hover(&restart_button.style) {
                    TypingTestFocus::RestartButton
                } else if util::is_hover(&theme_button.style) {
                    TypingTestFocus::ThemeButton
                } else if util::is_hover(&next_button.style) {
                    TypingTestFocus::NextButton
                } else {
                    TypingTestFocus::Nothing
                };
                state.dispatch(TypingtestAction::FocusChange(f));
            }
            _ => (),
        }

        let index = *typingbox.state.get().index.borrow();
        let len = typingbox.state.get().letters.borrow().len();
        let wpm = *typingbox
            .state
            .get()
            .incremental_wpm
            .borrow()
            .last()
            .unwrap_or(&0);

        window::clear_background(*app.style.theme.bg.borrow());

        typingbox.update();
        tracker.update(index, len, wpm);

        if *state.get().focus.borrow() != TypingTestFocus::TypingBox {
            next_button.update();
            restart_button.update();
            theme_button.update();
        }

        match *state.get().focus.borrow() {
            TypingTestFocus::ThemeButton => theme_button.style.draw_border(),
            TypingTestFocus::RestartButton => restart_button.style.draw_border(),
            TypingTestFocus::NextButton => next_button.style.draw_border(),
            _ => (),
        }

        window::next_frame().await;
    }
}
