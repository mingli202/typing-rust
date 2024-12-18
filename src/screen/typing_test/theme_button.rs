use std::sync::Arc;

use macroquad::text::TextDimensions;
use macroquad::{text, window};

use crate::screen::{self, theme::Theme, BorderParams, Style, Value};

pub struct ThemeButton {
    pub style: Style,
    pub text: String,
}

impl ThemeButton {
    pub fn new(style: &Style) -> Self {
        let text = "Theme".to_string();

        let TextDimensions { width, .. } =
            text::measure_text(&text, None, *style.font_size.lock().unwrap() as u16, 1.0);
        let font_size = Arc::clone(&style.font_size);
        let f2 = Arc::clone(&style.font_size);
        let f3 = Arc::clone(&style.font_size);

        ThemeButton {
            text,
            style: Style {
                border: Some(BorderParams {
                    size: 2.0,
                    color: Arc::clone(&style.theme.text),
                }),
                x: Value::Relative(Box::new(move || {
                    (window::screen_width() - width - 20.0) / 2.0
                })),
                y: Value::Relative(Box::new(move || {
                    (window::screen_height() - *font_size.lock().unwrap() * 3.0) / 2.0
                        - 10.0
                        - 3.0 * *font_size.lock().unwrap()
                })),
                width: Value::Relative(Box::new(move || {
                    text::measure_text("Theme", None, *f2.lock().unwrap() as u16, 1.0).width + 20.0
                })),
                height: Value::Relative(Box::new(move || {
                    text::measure_text("Theme", None, *f3.lock().unwrap() as u16, 1.0).height + 20.0
                })),
                font_size: Arc::clone(&style.font_size),
                theme: Theme {
                    bg: Arc::clone(&style.theme.bg),
                    ghost: Arc::clone(&style.theme.ghost),
                    text: Arc::clone(&style.theme.ghost),
                    error: Arc::clone(&style.theme.error),
                },
                padding_x: Some(Value::Absolute(10.0)),
                padding_y: Some(Value::Absolute(10.0)),
                ..Style::default()
            },
        }
    }

    pub fn update(&self) {
        screen::text::print_text(
            &self.style,
            &self.text,
            self.style.x.get(),
            self.style.y.get(),
        );
    }
}
