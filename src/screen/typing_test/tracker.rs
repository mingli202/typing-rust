use std::sync::Arc;

use macroquad::window;

use crate::screen::{text, theme::Theme, Style, Value};

pub struct Tracker {
    style: Style,
}

impl Tracker {
    pub fn new(style: &Style) -> Tracker {
        let font_size = Arc::clone(&style.font_size);

        Tracker {
            style: Style {
                x: Value::Relative(Box::new(|| (0.5 * window::screen_width()) / 2.0)),
                y: Value::Relative(Box::new(move || {
                    (window::screen_height() - *font_size.lock().unwrap() * 3.0) / 2.0
                        - *font_size.lock().unwrap()
                })),
                theme: Theme {
                    bg: Arc::clone(&style.theme.bg),
                    ghost: Arc::clone(&style.theme.ghost),
                    text: Arc::clone(&style.theme.ghost),
                    error: Arc::clone(&style.theme.error),
                },
                font_size: Arc::clone(&style.font_size),
                ..Style::default()
            },
        }
    }

    pub fn update(&self, index: usize, len: usize) {
        text::print_text(
            &self.style,
            &format!("{}/{}", index, len),
            self.style.x.get(),
            self.style.y.get(),
        );
    }
}
