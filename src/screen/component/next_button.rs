use std::rc::Rc;

use macroquad::{text, window};

use crate::screen::theme::Theme;

use super::{BorderParams, Style, Value};

pub struct NextButtonState {}

pub struct NextButton {
    pub text: String,
    pub style: Style,
}

impl NextButton {
    pub fn new(style: &Style) -> NextButton {
        let text = "Next (n)".to_string();

        let dim = text::measure_text(&text, None, style.font_size as u16, 1.0);
        let width = dim.width;
        let o_y = dim.offset_y;
        let font_size = style.font_size;

        NextButton {
            text: text.to_string(),
            style: Style {
                border: Some(BorderParams {
                    size: 2.0,
                    color: Rc::clone(&style.theme.text),
                }),
                x: Value::Relative(Box::new(move || {
                    window::screen_width() / 2.0 - width - 10.0
                })),
                y: Value::Relative(Box::new(move || {
                    (window::screen_height()) / 2.0 + font_size
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
                offset_y: Some(Value::Absolute(o_y)),
                width: Value::Absolute(width + 20.0),
                height: Value::Absolute(font_size + 10.0),
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
