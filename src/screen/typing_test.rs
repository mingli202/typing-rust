use macroquad::input::{self, KeyCode};
use macroquad::window;

use super::component::restart_button::RestartButton;
use super::component::textbox::TextBox;
use super::component::theme_button::ThemeButton;
use super::component::tracker::Tracker;
use super::{Screen, State};

#[derive(PartialEq)]
enum Focus {
    RestartButton,
    ThemeButton,
    TypingBox,
    Nothing,
}

impl Focus {
    fn next(&mut self) {
        match self {
            Focus::Nothing => *self = Focus::RestartButton,
            Focus::TypingBox => *self = Focus::RestartButton,
            Focus::RestartButton => *self = Focus::ThemeButton,
            Focus::ThemeButton => *self = Focus::RestartButton,
        }
    }
}

pub async fn run(scr: &mut Screen, wpm: &mut u16) -> State {
    assert_eq!(scr.state, State::TypingTest);

    let mut focus = Focus::Nothing;

    let mut typingbox: TextBox = TextBox::new(&scr.style, &scr.data);

    let tracker = Tracker::new(&scr.style);

    let mut restart_button = RestartButton::new(&scr.style);

    let theme_button = ThemeButton::new(&scr.style);

    loop {
        if let Some(k) = input::get_last_key_pressed() {
            match k {
                KeyCode::Enter => {
                    input::clear_input_queue();
                    match focus {
                        Focus::RestartButton => {
                            typingbox.refresh();
                            focus = Focus::Nothing;
                        }
                        Focus::ThemeButton => {
                            return State::ThemeSelect;
                        }
                        _ => (),
                    }
                }

                KeyCode::Tab => {
                    input::clear_input_queue();
                    focus.next();
                }
                KeyCode::Backspace => {
                    input::clear_input_queue();
                    focus = Focus::TypingBox;
                    typingbox.delete_char();
                }
                // this passes the keytrokes to type
                _ => {
                    if let Some(c) = input::get_char_pressed() {
                        focus = Focus::TypingBox;

                        if typingbox.on_type(c) {
                            *wpm = typingbox.get_wpm();
                            return State::EndScreen;
                        }
                    }
                }
            }
        }

        window::clear_background(*scr.style.theme.bg.borrow());

        typingbox.update();
        tracker.update(typingbox.state.index, typingbox.state.letters.len());

        if focus != Focus::TypingBox {
            restart_button.update();
            theme_button.update();
        }

        match focus {
            Focus::ThemeButton => theme_button.style.draw_border(),
            Focus::RestartButton => restart_button.style.draw_border(),
            _ => (),
        }

        window::next_frame().await;
    }
}
