use std::rc::Rc;

use macroquad::{
    text::{self, TextDimensions},
    window,
};

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

        let TextDimensions {
            width,
            height,
            offset_y,
        } = text::measure_text(&text, None, style.font_size as u16, 1.0);
        let font_size = style.font_size;

        CancelButton {
            text: text.to_string(),
            style: Style {
                border: Some(BorderParams {
                    size: 2.0,
                    color: Rc::clone(&style.theme.text),
                }),
                x: Value::Relative(Box::new(move || (window::screen_width() - width) / 2.0)),
                y: Value::Relative(Box::new(move || {
                    (window::screen_height() + font_size) / 2.0
                })),
                font_size: style.font_size,
                theme: Theme {
                    bg: Rc::clone(&style.theme.bg),
                    ghost: Rc::clone(&style.theme.ghost),
                    text: Rc::clone(&style.theme.ghost),
                    error: Rc::clone(&style.theme.error),
                },
                padding_x: Some(Value::Absolute(10.0)),
                padding_y: Some(Value::Absolute(10.0)),
                offset_y: Some(Value::Absolute(offset_y)),
                width: Value::Absolute(width + 20.0),
                height: Value::Absolute(height + 20.0),
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
