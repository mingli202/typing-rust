use std::sync::Arc;

use macroquad::{text, window};

use crate::screen::{self, theme::Theme, BorderParams, Style, Value};

pub struct QuitButton {
    pub style: Style,

    pub text: String,
}

impl QuitButton {
    pub fn new(style: &Style) -> QuitButton {
        let text = "Quit (q)".to_string();

        let font_size = Arc::clone(&style.font_size);
        let f1 = Arc::clone(&style.font_size);
        let f3 = Arc::clone(&style.font_size);
        let f4 = Arc::clone(&style.font_size);

        QuitButton {
            text: text.to_string(),
            style: Style {
                border: Some(BorderParams {
                    size: 2.0,
                    color: Arc::clone(&style.theme.text),
                }),
                x: Value::Relative(Box::new(move || {
                    let f1 = *f1.lock().unwrap();
                    (window::screen_width() + f1) / 2.0
                })),
                y: Value::Relative(Box::new(move || {
                    let font_size = *font_size.lock().unwrap();
                    (window::screen_height() + font_size) / 2.0
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
                width: Value::Relative(Box::new(move || {
                    let f3 = *f3.lock().unwrap();
                    text::measure_text("Next (n)", None, f3 as u16, 1.0).width + 20.0
                })),
                height: Value::Relative(Box::new(move || {
                    let f4 = *f4.lock().unwrap();
                    text::measure_text("Next (n)", None, f4 as u16, 1.0).height + 20.0
                })),
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
