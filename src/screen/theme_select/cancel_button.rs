use std::sync::Arc;

use macroquad::{text, window};

use crate::screen::Value;
use crate::screen::{
    style::{BorderParams, Style},
    theme::Theme,
};

pub struct CancelButton {
    pub text: String,
    pub style: Style,
}

impl CancelButton {
    pub fn new(style: &Style) -> CancelButton {
        let text = "Cancel (ESC)".to_string();

        let font_size = Arc::clone(&style.font_size);
        let f1 = Arc::clone(&style.font_size);
        let f2 = Arc::clone(&style.font_size);
        let f3 = Arc::clone(&style.font_size);

        CancelButton {
            text: text.to_string(),
            style: Style {
                border: Some(BorderParams {
                    size: 2.0,
                    color: Arc::clone(&style.theme.text),
                }),
                x: Value::Relative(Box::new(move || {
                    (window::screen_width()
                        - text::measure_text(&text, None, *f3.lock().unwrap() as u16, 1.0).width)
                        / 2.0
                })),
                y: Value::Relative(Box::new(move || {
                    (window::screen_height() + *font_size.lock().unwrap()) / 2.0
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
                    text::measure_text("Cancel (ESC)", None, *f1.lock().unwrap() as u16, 1.0).width
                        + 20.0
                })),
                height: Value::Relative(Box::new(move || {
                    text::measure_text("Cancel (ESC)", None, *f2.lock().unwrap() as u16, 1.0).height
                        + 20.0
                })),
                ..Style::default()
            },
        }
    }

    pub fn update(&self) {
        crate::screen::text::print_text(
            &self.style,
            &self.text,
            self.style.x.get(),
            self.style.y.get(),
        );
    }
}
