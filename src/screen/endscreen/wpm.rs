use std::sync::Arc;

use macroquad::text::{self, TextDimensions};
use macroquad::window;

use crate::screen::{theme::Theme, Style, Value};

pub struct Wpm {
    pub wpm: String,
    pub style: Style,
}

impl Wpm {
    pub fn new(style: &Style, wmp: u16) -> Wpm {
        let f1 = Arc::clone(&style.font_size);
        let f2 = Arc::clone(&style.font_size);

        Wpm {
            wpm: format!("WPM: {}", wmp),
            style: Style {
                x: Value::Relative(Box::new(move || {
                    let f = *f1.lock().unwrap();
                    (window::screen_width()
                        - text::measure_text(&format!("WPM: {}", wmp), None, f as u16, 1.0).width)
                        / 2.0
                })),
                y: Value::Relative(Box::new(move || {
                    let f = *f2.lock().unwrap();
                    let TextDimensions {
                        height, offset_y, ..
                    } = text::measure_text(&format!("WPM: {}", wmp), None, f as u16, 1.0);
                    (window::screen_height() - height + offset_y - f) / 2.0
                })),
                font_size: Arc::clone(&style.font_size),
                theme: Theme {
                    bg: Arc::clone(&style.theme.bg),
                    ghost: Arc::clone(&style.theme.ghost),
                    text: Arc::clone(&style.theme.text),
                    error: Arc::clone(&style.theme.error),
                },
                ..Style::default()
            },
        }
    }

    pub fn update(&self) {
        let f = *self.style.font_size.lock().unwrap();
        let t = *self.style.theme.text.lock().unwrap();

        macroquad::text::draw_text(&self.wpm, self.style.x.get(), self.style.y.get(), f, t);
    }
}
