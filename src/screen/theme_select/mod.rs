use std::sync::Arc;

use macroquad::input::{KeyCode, MouseButton};
use macroquad::text::TextDimensions;
use macroquad::{input, text, window};

use super::style::BorderParams;
use super::theme::ThemeName::*;
use super::{util, Screen, State, Value};

mod button;
mod cancel_button;

pub async fn run(scr: &mut Screen) -> State {
    let current = scr.config.theme.clone();

    let mut focus = -1;

    let themes = [Atom, Catppuccin, Gruvbox, Tokyonight];
    let mut buttons = themes.map(|t| button::Button::new(t, &scr.style));

    let cancel_button = cancel_button::CancelButton::new(&scr.style);

    // to deal with holding
    let mut is_mouse_held = true;

    loop {
        if let Some(k) = input::get_last_key_pressed() {
            match k {
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
                KeyCode::Tab => {
                    if input::is_key_down(KeyCode::LeftShift)
                        || input::is_key_down(KeyCode::RightShift)
                    {
                        // cycle to -1 as well
                        if focus == -1 {
                            focus = buttons.len() as i32 - 1;
                        } else {
                            focus -= 1;
                        }
                    } else if focus == buttons.len() as i32 - 1 {
                        focus = -1;
                    } else {
                        focus = (focus + 1) % buttons.len() as i32;
                    }
                }
                KeyCode::Escape => {
                    scr.style.theme.set(&current);
                    return State::TypingTest;
                }
                _ => {
                    if let Some(c) = input::get_char_pressed() {
                        // enter
                        if c == '\u{000d}' {
                            if focus >= 0 {
                                scr.config.theme = buttons[focus as usize].theme_name.clone();
                                scr.config.update_file();
                            }
                            return State::TypingTest;
                        }
                    }
                }
            }
        }

        let bg = *scr.style.theme.bg.lock().unwrap();
        window::clear_background(bg);

        let mut x = 0.25 * window::screen_width();
        let mut y = 0.24 * window::screen_height();

        for (i, button) in buttons.iter_mut().enumerate() {
            let font_size = *button.style.font_size.lock().unwrap();
            let TextDimensions { width, .. } =
                text::measure_text(&button.text, None, font_size as u16, 1.0);

            if x + width > 0.75 * window::screen_width() {
                y += font_size + 30.0;
                x = 0.25 * window::screen_width();
            }

            button.style.x = Value::Absolute(x);
            button.style.y = Value::Absolute(y);

            if i as i32 == focus {
                focus = i as i32;
                *button.style.border.as_mut().unwrap() = BorderParams {
                    size: 2.0,
                    color: Arc::clone(&button.style.theme.text),
                }
            } else {
                *button.style.border.as_mut().unwrap() = BorderParams {
                    size: 2.0,
                    color: Arc::clone(&button.style.theme.ghost),
                }
            }

            button.update();
            x += width + 50.0;
        }

        cancel_button.update();

        if util::is_hover(&cancel_button.style) || focus == -1 {
            focus = -1;
            cancel_button.style.draw_border();
            scr.style.theme.set(&current);
        }

        window::next_frame().await;

        for (i, button) in buttons.iter().enumerate() {
            if util::is_hover(&button.style) {
                focus = i as i32;
            }
        }

        if focus >= 0 {
            scr.style.theme.set(&buttons[focus as usize].theme_name);
        }

        if input::is_mouse_button_down(MouseButton::Left) {
            if !is_mouse_held {
                if focus >= 0 && util::is_hover(&buttons[focus as usize].style) {
                    scr.config.theme = buttons[focus as usize].theme_name.clone();
                    scr.config.update_file();
                    return State::TypingTest;
                } else if focus != -2 {
                    scr.style.theme.set(&current);
                    return State::TypingTest;
                }
            }
            is_mouse_held = true;
        } else {
            is_mouse_held = false;
        }
    }
}
