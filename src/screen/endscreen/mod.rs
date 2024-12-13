use std::process;

use macroquad::input::{self, KeyCode, MouseButton};
use macroquad::math::Vec2;
use macroquad::window;

mod next_button;
mod quit_button;
mod wpm;

use super::{util, Screen, State};

enum Focus {
    NextButton,
    QuitButton,
    Nothing,
}

impl Focus {
    fn next(&mut self) {
        match self {
            Focus::Nothing => *self = Focus::NextButton,
            Focus::NextButton => *self = Focus::QuitButton,
            Focus::QuitButton => *self = Focus::NextButton,
        }
    }
}

pub async fn run(scr: &mut Screen, wpm: &u16) -> State {
    let mut focus = Focus::Nothing;

    let next_button = next_button::NextButton::new(&scr.style);
    let quit_button = quit_button::QuitButton::new(&scr.style);
    let wpm = wpm::Wpm::new(&scr.style, *wpm);

    loop {
        if let Some(k) = input::get_last_key_pressed() {
            match k {
                KeyCode::Tab => focus.next(),
                KeyCode::Enter => match focus {
                    Focus::NextButton => return State::TypingTest,
                    Focus::QuitButton => process::exit(0),
                    _ => (),
                },
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
                Focus::NextButton => return State::TypingTest,
                Focus::QuitButton => process::exit(0),
                _ => (),
            }
        }

        match input::mouse_delta_position() {
            Vec2 { x: dx, y: dy } if dx != 0.0 && dy != 0.0 => {
                focus = if util::is_hover(&next_button.style) {
                    Focus::NextButton
                } else if util::is_hover(&quit_button.style) {
                    Focus::QuitButton
                } else {
                    Focus::Nothing
                }
            }
            _ => (),
        }

        window::clear_background(*scr.style.theme.bg.borrow());

        next_button.update();
        quit_button.update();
        wpm.update();

        match focus {
            Focus::QuitButton => quit_button.style.draw_border(),
            Focus::NextButton => next_button.style.draw_border(),
            _ => (),
        }

        window::next_frame().await;
    }
}
