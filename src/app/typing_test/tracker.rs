use std::rc::Rc;

use macroquad::text::Font;
use macroquad::window;

use crate::app::text::PrintOptions;
use crate::app::{text, theme::Theme, Style, Value};

pub struct Tracker {
    style: Style,
    font: Rc<Font>,
}

impl Tracker {
    pub fn new(style: &Style, font: Rc<Font>) -> Tracker {
        let font_size = Rc::clone(&style.font_size);

        Tracker {
            style: Style {
                x: Value::Relative(Box::new(|_| (0.5 * window::screen_width()) / 2.0)),
                y: Value::Relative(Box::new(move |_| {
                    (window::screen_height() - *font_size.borrow() * 3.0) / 2.0
                        - *font_size.borrow()
                })),
                theme: Theme {
                    bg: Rc::clone(&style.theme.bg),
                    ghost: Rc::clone(&style.theme.ghost),
                    text: Rc::clone(&style.theme.ghost),
                    error: Rc::clone(&style.theme.error),
                },
                font_size: Rc::clone(&style.font_size),
                ..Style::default()
            },
            font,
        }
    }

    pub fn update(&self, index: usize, len: usize, wpm: u16) {
        text::print_text(
            &self.style,
            &format!("{}/{} {}", index, len, wpm),
            PrintOptions {
                font: Some(Rc::clone(&self.font)),
                ..PrintOptions::default()
            },
        );
    }
}
