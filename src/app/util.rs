use macroquad::color::Color;
use macroquad::input::KeyCode;
use macroquad::{input, shapes, window};

use crate::Config;

use super::state::screen::{AppAction, AppState};
use super::state::State;
use super::Style;

pub fn is_hover(style: &Style) -> bool {
    let (x, y) = input::mouse_position();

    if style.x.get() <= x
        && x <= style.x.get() + style.width.get()
        && style.y.get() <= y
        && y <= style.y.get() + style.height.get()
    {
        return true;
    }

    false
}

pub fn handle_resize(state: &State<AppState, AppAction>, config: &Config) -> Result<(), ()> {
    if let Some(k) = input::get_last_key_pressed() {
        let font_change = match k {
            KeyCode::Equal
                if (input::is_key_down(KeyCode::LeftSuper)
                    || input::is_key_down(KeyCode::RightSuper)) =>
            {
                5.0
            }
            KeyCode::Minus
                if (input::is_key_down(KeyCode::LeftSuper)
                    || input::is_key_down(KeyCode::RightSuper)) =>
            {
                -5.0
            }
            KeyCode::Key0
                if (input::is_key_down(KeyCode::LeftSuper)
                    || input::is_key_down(KeyCode::RightSuper)) =>
            {
                config.font_size
            }
            _ => return Err(()),
        };

        input::clear_input_queue();
        state.dispatch(AppAction::FontChange(font_change));
        Ok(())
    } else {
        Err(())
    }
}

#[allow(unused)]
pub fn draw_midpoint() {
    shapes::draw_line(
        window::screen_width() / 2.0,
        0.0,
        window::screen_width() / 2.0,
        window::screen_height(),
        1.0,
        Color::new(1.0, 0.0, 0.0, 1.0),
    );

    shapes::draw_line(
        0.0,
        window::screen_height() / 2.0,
        window::screen_width(),
        window::screen_height() / 2.0,
        1.0,
        Color::new(1.0, 0.0, 0.0, 1.0),
    );
}
