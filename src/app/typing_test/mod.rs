use macroquad::input::{self, KeyCode, MouseButton};
use macroquad::math::Vec2;
use macroquad::window;

use crate::app::focus::{Focus, TypingTestFocus::*};
use crate::app::util;

mod mode_select;
mod next_button;
mod restart_button;
mod textbox;
mod theme_button;
mod tracker;

use super::{App, Screen};

pub async fn run(app: &mut App) {
    input::clear_input_queue();

    let mut focus = Nothing;
    let mut is_mode_hover = false;

    let mut typingbox = textbox::TextBox::new(&app.style, app.state.mode.get_inner().clone());
    let tracker = tracker::Tracker::new(&app.style);
    let next_button = next_button::NextButton::new(&app.style);
    let restart_button = restart_button::RestartButton::new(&app.style);
    let theme_button = theme_button::ThemeButton::new(&app.style);

    loop {
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
                    *app.style.font_size.borrow_mut() += 5.0;
                }
                KeyCode::Minus
                    if (input::is_key_down(KeyCode::LeftSuper)
                        || input::is_key_down(KeyCode::RightSuper)) =>
                {
                    input::clear_input_queue();
                    *app.style.font_size.borrow_mut() -= 5.0;
                }
                KeyCode::Key0
                    if (input::is_key_down(KeyCode::LeftSuper)
                        || input::is_key_down(KeyCode::RightSuper)) =>
                {
                    input::clear_input_queue();
                    *app.style.font_size.borrow_mut() = app.config.font_size;
                }
                KeyCode::Enter => {
                    input::clear_input_queue();
                    match focus {
                        NextButton => {
                            app.state.mode.next(&app.data);
                            typingbox.refresh(app.state.mode.get_inner());
                        }
                        RestartButton => {
                            typingbox.refresh(app.state.mode.get_inner());
                        }
                        ThemeButton => {
                            app.state.screen = Screen::ThemeSelect;
                            return;
                        }
                        _ => (),
                    }
                }
                KeyCode::Tab => {
                    input::clear_input_queue();
                    focus.next();
                }
                // this passes the keytrokes to type
                _ => {
                    if let Some(c) = input::get_char_pressed() {
                        focus = TypingBox;
                        if typingbox.on_type(c) {
                            app.state.wpm = typingbox.get_wpm();
                            app.state.screen = Screen::End;
                            return;
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
            match focus {
                NextButton => {
                    app.state.mode.next(&app.data);
                    typingbox.refresh(app.state.mode.get_inner());
                }
                RestartButton => {
                    typingbox.refresh(app.state.mode.get_inner());
                }
                ThemeButton => {
                    app.state.screen = Screen::ThemeSelect;
                    return;
                }
                _ => (),
            }
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

        window::clear_background(*app.style.theme.bg.borrow());

        typingbox.update();
        tracker.update(typingbox.state.index, typingbox.state.letters.len());

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
}
