use std::process;

use macroquad::input::{self, KeyCode, MouseButton};
use macroquad::math::Vec2;
use macroquad::window;

mod next_button;
mod quit_button;
mod restart_button;
mod wpm;

use super::focus::{
    EndscreenFocus::{self, *},
    Focus,
};
use super::{util, Mode, ReturnType, Screen, State};

// TODO: global state management
pub async fn run<'a>(scr: &'a Screen, wpm: u16, mut mode: Mode<'a>) -> ReturnType<'a> {
    input::show_mouse(true);

    let mut state = State::EndScreen;

    let mut focus = Nothing;
    let mut run = true;

    let next_button = next_button::NextButton::new(&scr.style);
    let quit_button = quit_button::QuitButton::new(&scr.style);
    let restart_button = restart_button::RestartButton::new(&scr.style);
    let wpm_indicator = wpm::Wpm::new(&scr.style, wpm);

    while run {
        if let Some(k) = input::get_last_key_pressed() {
            match k {
                KeyCode::Tab => focus.next(),
                KeyCode::Enter => {
                    handle_click(scr, &mut focus, &mut mode, &mut state, &mut run, ' ')
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
                _ => {
                    if let Some(c) = input::get_char_pressed() {
                        handle_click(scr, &mut focus, &mut mode, &mut state, &mut run, c);
                    }
                }
            }
        }

        if input::is_mouse_button_pressed(MouseButton::Left) {
            handle_click(scr, &mut focus, &mut mode, &mut state, &mut run, ' ');
        }

        match input::mouse_delta_position() {
            Vec2 { x: dx, y: dy } if dx != 0.0 && dy != 0.0 => {
                focus = if util::is_hover(&next_button.style) {
                    NextButton
                } else if util::is_hover(&quit_button.style) {
                    QuitButton
                } else if util::is_hover(&restart_button.style) {
                    RestartButton
                } else {
                    Nothing
                }
            }
            _ => (),
        }

        window::clear_background(*scr.style.theme.bg.borrow());

        next_button.update();
        quit_button.update();
        restart_button.update();
        wpm_indicator.update();

        match focus {
            QuitButton => quit_button.style.draw_border(),
            NextButton => next_button.style.draw_border(),
            RestartButton => restart_button.style.draw_border(),
            _ => (),
        }

        window::next_frame().await;
    }

    (state, wpm, mode)
}

fn handle_click<'a>(
    scr: &'a Screen,
    focus: &mut EndscreenFocus,
    mode: &mut Mode<'a>,
    state: &mut State,
    run: &mut bool,
    c: char,
) {
    input::clear_input_queue();

    match (focus, c) {
        (_, 'n') | (NextButton, _) => {
            mode.next(&scr.data);
            *state = State::TypingTest;
            *run = false;
        }
        (_, 'r') | (RestartButton, _) => {
            *state = State::TypingTest;
            *run = false
        }
        (_, 'q') | (QuitButton, _) => process::exit(0),
        _ => (),
    };
}
