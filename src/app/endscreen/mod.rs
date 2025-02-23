use std::process;
use std::rc::Rc;

use macroquad::input::{self, KeyCode, MouseButton};
use macroquad::math::Vec2;
use macroquad::window;

mod graph;
mod next_button;
mod quit_button;
mod restart_button;
mod source;
mod wpm;

use super::focus::{EndscreenFocus::*, Focus};
use super::{util, App, Screen};

pub async fn run(app: &mut App) {
    input::show_mouse(true);
    let mut focus = Nothing;

    let wpm = wpm::Wpm::new(&app.style, app.state.wpm, Rc::clone(&app.font));
    let next_button = next_button::NextButton::new(&app.style, Rc::clone(&app.font));
    let quit_button = quit_button::QuitButton::new(&app.style, Rc::clone(&app.font));
    let restart_button = restart_button::RestartButton::new(&app.style, Rc::clone(&app.font));
    let source = source::Source::new(app.state.mode.to_string(), &app.style, Rc::clone(&app.font));

    loop {
        if let Some(k) = input::get_last_key_pressed() {
            match k {
                KeyCode::Tab => focus.next(),
                KeyCode::Enter => match focus {
                    NextButton => {
                        app.state.mode.next(&app.data);
                        app.state.screen = Screen::TypingTest;
                        return;
                    }
                    RestartButton => {
                        app.state.screen = Screen::TypingTest;
                        return;
                    }
                    QuitButton => process::exit(0),
                    _ => (),
                },
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
                _ => {
                    if let Some(c) = input::get_char_pressed() {
                        match c {
                            'n' => {
                                app.state.mode.next(&app.data);
                                app.state.screen = Screen::TypingTest;
                                return;
                            }
                            'r' => {
                                app.state.screen = Screen::TypingTest;
                                return;
                            }
                            'q' => process::exit(0),
                            _ => (),
                        }
                    }
                }
            }
        }

        if input::is_mouse_button_pressed(MouseButton::Left) {
            match focus {
                NextButton => {
                    app.state.mode.next(&app.data);
                    app.state.screen = Screen::TypingTest;
                    return;
                }
                RestartButton => {
                    app.state.screen = Screen::TypingTest;
                    return;
                }
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
                } else if util::is_hover(&restart_button.style) {
                    RestartButton
                } else {
                    Nothing
                }
            }
            _ => (),
        }

        window::clear_background(*app.style.theme.bg.borrow());

        next_button.update();
        quit_button.update();
        restart_button.update();
        wpm.update();
        source.update();

        match focus {
            QuitButton => quit_button.style.draw_border(),
            NextButton => next_button.style.draw_border(),
            RestartButton => restart_button.style.draw_border(),
            _ => (),
        }

        util::draw_midpoint();

        window::next_frame().await;
    }
}
