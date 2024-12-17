use std::rc::Rc;

use macroquad::text::TextDimensions;
use macroquad::{text, window};

use crate::screen::{self, theme::Theme};

use crate::screen::{BorderParams, Style, Value};

pub struct RestartButton {
    pub style: Style,
    pub text: String,
}

impl RestartButton {
    pub fn new(style: &Style) -> RestartButton {
        let text = "Restart".to_string();

        let TextDimensions {
            width,
            offset_y,
            height,
        } = text::measure_text(&text, None, *style.font_size.borrow() as u16, 1.0);
        let font_size = Rc::clone(&style.font_size);

        RestartButton {
            text: "Restart".to_string(),

            style: Style {
                border: Some(BorderParams {
                    size: 2.0,
                    color: Rc::clone(&style.theme.text),
                }),
                x: Value::Relative(Box::new(move || {
                    (window::screen_width() - width - 20.0) / 2.0
                })),
                y: Value::Relative(Box::new(move || {
                    (window::screen_height() + *font_size.borrow() * 3.0 + 10.0) / 2.0
                })),
                width: Value::Absolute(width + 20.0),
                height: Value::Absolute(height + 20.0),
                font_size: Rc::clone(&style.font_size),
                theme: Theme {
                    bg: Rc::clone(&style.theme.bg),
                    ghost: Rc::clone(&style.theme.ghost),
                    text: Rc::clone(&style.theme.ghost),
                    error: Rc::clone(&style.theme.error),
                },
                padding_x: Some(Value::Absolute(10.0)),
                padding_y: Some(Value::Absolute(10.0)),
                offset_y: Some(Value::Absolute(offset_y)),
                ..Style::default()
            },
        }
    }

    pub fn update(&mut self) {
        screen::text::print_text(
            &self.style,
            &self.text,
            self.style.x.get(),
            self.style.y.get(),
        );
    }
}
