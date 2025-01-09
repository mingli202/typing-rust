use std::rc::Rc;

use macroquad::input::{self, KeyCode, MouseButton};
use macroquad::math::Vec2;
use macroquad::window;

use crate::screen::focus::{Focus, TypingTestFocus::*};
use crate::screen::util;

mod next_button;
mod restart_button;
pub mod textbox;
mod theme_button;
mod tracker;

use super::state::{Action, State};

pub async fn run(state: &State) {
    input::clear_input_queue();

    let mode = Rc::clone(&state.mode);
    let focus = Rc::clone(&state.typingtest.focus);

    let mut typingbox = textbox::TextBox::new(&state.style, mode.borrow().get_text());
    let tracker = tracker::Tracker::new(&state.style);
    let next_button = next_button::NextButton::new(&state.style);
    let restart_button = restart_button::RestartButton::new(&state.style);
    let theme_button = theme_button::ThemeButton::new(&state.style);

    loop {
        if let Some(k) = input::get_last_key_pressed() {
            util::handle_resize(state);

            match k {
                KeyCode::Backspace => {
                    input::clear_input_queue();
                    state.dispatch(Action::TypingTestFocusChange(TypingBox));
                    typingbox.delete_char();
                }
                KeyCode::Enter => state.dispatch(Action::TypingtestClick(&mut typingbox)),
                KeyCode::Tab => {
                    input::clear_input_queue();
                    focus.borrow_mut().next();
                }
                // this passes the keytrokes to type
                _ => {
                    if let Some(c) = input::get_char_pressed() {
                        state.dispatch(Action::TypingTestFocusChange(TypingBox));

                        if typingbox.on_type(c) {
                            state.dispatch(Action::WpmChange(typingbox.get_wpm(None)));
                            state.dispatch(Action::ScreenChange(super::state::Screen::EndScreen));
                        }
                    }
                }
            }
        }

        if *focus.borrow() == TypingBox {
            input::show_mouse(false);
        } else {
            input::show_mouse(true);
        }

        if input::is_mouse_button_pressed(MouseButton::Left) {
            state.dispatch(Action::TypingtestClick(&mut typingbox))
        }

        match input::mouse_delta_position() {
            Vec2 { x: dx, y: dy } if dx != 0.0 && dy != 0.0 => {
                let f = if util::is_hover(&restart_button.style) {
                    RestartButton
                } else if util::is_hover(&theme_button.style) {
                    ThemeButton
                } else if util::is_hover(&next_button.style) {
                    NextButton
                } else {
                    Nothing
                };
                state.dispatch(Action::TypingTestFocusChange(f));
            }
            _ => (),
        }

        window::clear_background(*state.style.theme.bg.borrow());

        typingbox.update();
        tracker.update(
            typingbox.state.index,
            typingbox.state.letters.len(),
            typingbox.state.incemental_wpm.last().unwrap_or(&0),
        );

        if *focus.borrow() != TypingBox {
            next_button.update();
            restart_button.update();
            theme_button.update();
        }

        match *focus.borrow() {
            ThemeButton => theme_button.style.draw_border(),
            RestartButton => restart_button.style.draw_border(),
            NextButton => next_button.style.draw_border(),
            _ => (),
        }

        window::next_frame().await;
    }
}
