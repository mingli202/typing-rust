use macroquad::input::{self, KeyCode, MouseButton};
use macroquad::math::Vec2;
use macroquad::window;

use crate::screen::focus::{
    Focus,
    TypingTestFocus::{self, *},
};
use crate::screen::util;

mod next_button;
mod restart_button;
mod textbox;
mod theme_button;
mod tracker;

use super::{Mode, ReturnType, Screen, State};

pub async fn run<'a>(scr: &'a Screen, mut wpm: u16, mut mode: Mode<'a>) -> ReturnType<'a> {
    input::clear_input_queue();

    let mut state = State::TypingTest;
    let mut focus = Nothing;

    let mut typingbox = textbox::TextBox::new(&scr.style, mode.get_text(), &scr.data);
    let tracker = tracker::Tracker::new(&scr.style);
    let next_button = next_button::NextButton::new(&scr.style);
    let restart_button = restart_button::RestartButton::new(&scr.style);
    let theme_button = theme_button::ThemeButton::new(&scr.style);

    let mut run = true;

    while run {
        if let Some(k) = input::get_last_key_pressed() {
            match k {
                KeyCode::Backspace => {
                    input::clear_input_queue();
                    focus = TypingBox;
                    typingbox.delete_char();
                }
                KeyCode::Equal
                    if (input::is_key_down(KeyCode::LeftSuper)
                        || input::is_key_down(KeyCode::RightSuper)) =>
                {
                    input::clear_input_queue();
                    *scr.style.font_size.borrow_mut() += 5.0;
                }
                KeyCode::Minus
                    if (input::is_key_down(KeyCode::LeftSuper)
                        || input::is_key_down(KeyCode::RightSuper)) =>
                {
                    input::clear_input_queue();
                    *scr.style.font_size.borrow_mut() -= 5.0;
                }
                KeyCode::Key0
                    if (input::is_key_down(KeyCode::LeftSuper)
                        || input::is_key_down(KeyCode::RightSuper)) =>
                {
                    input::clear_input_queue();
                    *scr.style.font_size.borrow_mut() = scr.config.font_size;
                }
                KeyCode::Enter => handle_click(
                    scr,
                    &mut focus,
                    &mut mode,
                    &mut state,
                    &mut run,
                    &mut typingbox,
                ),
                KeyCode::Tab => {
                    input::clear_input_queue();
                    focus.next();
                }
                // this passes the keytrokes to type
                _ => {
                    if let Some(c) = input::get_char_pressed() {
                        focus = TypingBox;
                        if typingbox.on_type(c) {
                            wpm = typingbox.get_wpm(None);
                            state = State::EndScreen;
                            run = false;
                        }
                    }
                }
            }
        }

        if focus == TypingBox {
            input::show_mouse(false);
        } else {
            input::show_mouse(true);
        }

        if input::is_mouse_button_pressed(MouseButton::Left) {
            handle_click(
                scr,
                &mut focus,
                &mut mode,
                &mut state,
                &mut run,
                &mut typingbox,
            );
        }

        match input::mouse_delta_position() {
            Vec2 { x: dx, y: dy } if dx != 0.0 && dy != 0.0 => {
                focus = if util::is_hover(&restart_button.style) {
                    RestartButton
                } else if util::is_hover(&theme_button.style) {
                    ThemeButton
                } else if util::is_hover(&next_button.style) {
                    NextButton
                } else {
                    Nothing
                }
            }
            _ => (),
        }

        window::clear_background(*scr.style.theme.bg.borrow());

        typingbox.update();
        tracker.update(
            typingbox.state.index,
            typingbox.state.letters.len(),
            typingbox.state.incemental_wpm.last().unwrap_or(&0),
        );

        if focus != TypingBox {
            next_button.update();
            restart_button.update();
            theme_button.update();
        }

        match focus {
            ThemeButton => theme_button.style.draw_border(),
            RestartButton => restart_button.style.draw_border(),
            NextButton => next_button.style.draw_border(),
            _ => (),
        }

        window::next_frame().await;
    }

    (state, wpm, mode)
}

fn handle_click<'a>(
    scr: &'a Screen,
    focus: &mut TypingTestFocus,
    mode: &mut Mode<'a>,
    state: &mut State,
    run: &mut bool,
    typingbox: &mut textbox::TextBox,
) {
    input::clear_input_queue();
    match focus {
        NextButton => {
            mode.next(&scr.data);
            typingbox.refresh(mode.get_text());
            *focus = Nothing;
        }
        RestartButton => {
            typingbox.refresh(mode.get_text());
            *focus = Nothing;
        }
        ThemeButton => {
            *state = State::ThemeSelect;
            *run = false;
        }
        _ => (),
    }
}
