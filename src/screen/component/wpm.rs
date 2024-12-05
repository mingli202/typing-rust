use std::rc::Rc;

use macroquad::text::TextDimensions;
use macroquad::window;

use crate::screen::{
    component::{Style, Value},
    theme::Theme,
};

pub struct Wpm {
    pub wmp: String,
    pub style: Style,
}

impl Wpm {
    pub fn new(style: &Style, wmp: u16) -> Wpm {
        let wmp = format!("WPM: {}", wmp);
        let TextDimensions {
            width,
            height,
            offset_y,
        } = macroquad::text::measure_text(&wmp, None, style.font_size as u16, 1.0);

        let font_size = style.font_size;

        Wpm {
            wmp,
            style: Style {
                x: Value::Relative(Box::new(move || (window::screen_width() - width) / 2.0)),
                y: Value::Relative(Box::new(move || {
                    (window::screen_height() - height + offset_y - font_size) / 2.0
                })),
                font_size: style.font_size,
                theme: Theme {
                    bg: Rc::clone(&style.theme.bg),
                    ghost: Rc::clone(&style.theme.ghost),
                    text: Rc::clone(&style.theme.text),
                    error: Rc::clone(&style.theme.error),
                },
                ..Style::default()
            },
        }
    }

    pub fn update(&self) {
        macroquad::text::draw_text(
            &self.wmp,
            self.style.x.get(),
            self.style.y.get(),
            self.style.font_size,
            *self.style.theme.text.borrow(),
        );
    }
}
