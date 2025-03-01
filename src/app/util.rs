#![allow(unused)]

use macroquad::color::Color;
use macroquad::{input, shapes, window};

use super::Style;

pub fn is_hover(style: &Style) -> bool {
    let (x, y) = input::mouse_position();

    if style.x.get(style) <= x
        && x <= style.x.get(style) + style.width.get(style)
        && style.y.get(style) <= y
        && y <= style.y.get(style) + style.height.get(style)
    {
        return true;
    }

    false
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
