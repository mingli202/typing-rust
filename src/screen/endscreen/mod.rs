use std::process;

use macroquad::input::{self, KeyCode, MouseButton};
use macroquad::math::Vec2;
use macroquad::window;

mod next_button;
mod quit_button;
mod wpm;

use super::focus::{EndscreenFocus::*, Focus};
use super::{util, Screen, State};

pub async fn run(scr: &mut Screen, wpm: &u16) -> State {
    let mut focus = Nothing;

    let next_button = next_button::NextButton::new(&scr.style);
    let quit_button = quit_button::QuitButton::new(&scr.style);
    let wpm = wpm::Wpm::new(&scr.style, *wpm);

    loop {
        if let Some(k) = input::get_last_key_pressed() {
            match k {
                KeyCode::Tab => focus.next(),
                KeyCode::Enter => match focus {
                    NextButton => return State::TypingTest,
                    QuitButton => process::exit(0),
                    _ => (),
                },
                KeyCode::Equal
                    if (input::is_key_down(KeyCode::LeftSuper)
                        || input::is_key_down(KeyCode::RightSuper)) =>
                {
                    input::clear_input_queue();
                    let mut font_size = scr.style.font_size.lock().unwrap();
                    *font_size += 5.0;
                }
                KeyCode::Minus
                    if (input::is_key_down(KeyCode::LeftSuper)
                        || input::is_key_down(KeyCode::RightSuper)) =>
                {
                    input::clear_input_queue();
                    let mut font_size = scr.style.font_size.lock().unwrap();
                    *font_size -= 5.0;
                }
                KeyCode::Key0
                    if (input::is_key_down(KeyCode::LeftSuper)
                        || input::is_key_down(KeyCode::RightSuper)) =>
                {
                    input::clear_input_queue();
                    let mut font_size = scr.style.font_size.lock().unwrap();
                    *font_size = scr.config.font_size;
                }
                _ => {
                    if let Some(c) = input::get_char_pressed() {
                        match c {
                            'n' => return State::TypingTest,
                            'q' => process::exit(0),
                            _ => (),
                        }
                    }
                }
            }
        }

        if input::is_mouse_button_pressed(MouseButton::Left) {
            match focus {
                NextButton => return State::TypingTest,
                QuitButton => process::exit(0),
                _ => (),
            }
        }

        match input::mouse_delta_position() {
            Vec2 { x: dx, y: dy } if dx != 0.0 && dy != 0.0 => {
                focus = if util::is_hover(&next_button.style) {
                    NextButton
                } else if util::is_hover(&quit_button.style) {
                    QuitButton
                } else {
                    Nothing
                }
            }
            _ => (),
        }

        let bg = *scr.style.theme.bg.lock().unwrap();
        window::clear_background(bg);

        next_button.update();
        quit_button.update();
        wpm.update();

        match focus {
            QuitButton => quit_button.style.draw_border(),
            NextButton => next_button.style.draw_border(),
            _ => (),
        }

        window::next_frame().await;
    }
}
