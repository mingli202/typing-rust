use std::process;

use macroquad::input::{self, KeyCode};
use macroquad::window;

use super::component::{next_button, quit_button, wpm};
use super::{Screen, State};

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
