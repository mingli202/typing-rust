use std::rc::Rc;

use macroquad::text::TextDimensions;
use macroquad::{input, text, window};

use super::style::BorderParams;
use super::theme::ThemeName::*;
use super::{Screen, State, Value};

mod button;

pub async fn run(scr: &mut Screen) -> State {
    let current = scr.theme_name.clone();

    let mut focus = -1;

    let themes = [Atom, Catppuccin, Gruvbox, Tokyonight];
    let mut buttons = themes.map(|t| button::Button::new(t, &scr.style));

    loop {
        window::clear_background(*scr.style.theme.bg.borrow());

        if let Some(k) = input::get_char_pressed() {
            match k {
                // tab
                '\u{0009}' => {
                    focus = (focus + 1) % buttons.len() as i32;
                    if focus >= 0 {
                        scr.style.theme.set(&buttons[focus as usize].theme_name);
                    }
                }
                // enter
                '\u{000d}' => return State::TypingTest,
                // escape
                '\u{001b}' => {
                    scr.style.theme.set(&current);
                    return State::TypingTest;
                }
                _ => (),
            }
        }

        let mut x = 0.25 * window::screen_width();
        let mut y = 0.24 * window::screen_height();

        for (i, button) in buttons.iter_mut().enumerate() {
            let TextDimensions { width, .. } =
                text::measure_text(&button.text, None, button.style.font_size as u16, 1.0);

            if x + width > 0.75 * window::screen_width() {
                y += button.style.font_size + 30.0;
                x = 0.25 * window::screen_width();
            }

            button.style.x = Value::Absolute(x);
            button.style.y = Value::Absolute(y);

            button.update();

            if i as i32 == focus {
                *button.style.border.as_mut().unwrap() = BorderParams {
                    size: 2.0,
                    color: Rc::clone(&button.style.theme.text),
                }
            } else {
                *button.style.border.as_mut().unwrap() = BorderParams {
                    size: 2.0,
                    color: Rc::clone(&button.style.theme.ghost),
                }
            }

            x += width + 50.0
        }

        window::next_frame().await;
    }
}
