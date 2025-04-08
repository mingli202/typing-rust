#![allow(unused)]

use macroquad::color::Color;
use macroquad::{input, shapes, window};

use super::Style;

pub fn is_hover(style: &Style) -> bool {
    let (x, y) = input::mouse_position();

    style.x() <= x
        && x <= style.x() + style.width()
        && style.y() <= y
        && y <= style.y() + style.height()
}

pub fn clamp<T: PartialOrd>(min: T, val: T, max: T) -> T {
    if val < min {
        min
    } else if val > max {
        max
    } else {
        val
    }
}

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
