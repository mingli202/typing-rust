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
                y: Value::Relative(Box::new(move |_| -*font_size.borrow() * 1.5)),
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

    pub fn update(&self, typingbox_style: &Style, index: usize, len: usize, wpm: u16) {
        text::print_text(
            &self.style,
            &format!("{}/{} {}", index, len, wpm),
            PrintOptions {
                x: Some(typingbox_style.x()),
                y: Some(typingbox_style.y() + self.style.y()),
                font: Some(Rc::clone(&self.font)),
                ..PrintOptions::default()
            },
        );
    }
}
