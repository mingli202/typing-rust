use std::rc::Rc;

use macroquad::window;

use crate::screen::{text, theme::Theme};

use super::{Component, Style, Value};

pub struct Tracker {
    len: usize,
    style: Style,
}

impl Component for Tracker {}

impl Tracker {
    pub fn new(style: &Style, len: usize) -> Tracker {
        let font_size = style.font_size;

        Tracker {
            style: Style {
                x: Value::Relative(Box::new(|| (0.5 * window::screen_width()) / 2.0)),
                y: Value::Relative(Box::new(move || {
                    (window::screen_height() - font_size * 3.0) / 2.0 - font_size
                })),
                theme: Theme {
                    bg: Rc::clone(&style.theme.bg),
                    ghost: Rc::clone(&style.theme.ghost),
                    text: Rc::clone(&style.theme.ghost),
                    error: Rc::clone(&style.theme.error),
                },
                font_size: style.font_size,
                ..Style::default()
            },
            len,
        }
    }

    pub fn update(&self, index: usize) {
        text::print_text(
            &self.style,
            &format!("{}/{}", index, self.len),
            self.style.x.get(),
            self.style.y.get(),
        );
    }
}
