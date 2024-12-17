use std::rc::Rc;

use macroquad::text::TextDimensions;
use macroquad::{text, window};

use crate::screen::{self, theme::Theme, BorderParams, Style, Value};

pub struct QuitButton {
    pub style: Style,

    pub text: String,
}

impl QuitButton {
    pub fn new(style: &Style) -> QuitButton {
        let text = "Quit (q)".to_string();

        let TextDimensions {
            height,
            width,
            offset_y,
        } = text::measure_text(&text, None, *style.font_size.borrow() as u16, 1.0);
        let font_size = Rc::clone(&style.font_size);

        QuitButton {
            text: text.to_string(),
            style: Style {
                border: Some(BorderParams {
                    size: 2.0,
                    color: Rc::clone(&style.theme.text),
                }),
                x: Value::Relative(Box::new(move || window::screen_width() / 2.0 + 10.0)),
                y: Value::Relative(Box::new(move || {
                    (window::screen_height() + *font_size.borrow()) / 2.0
                })),
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
                width: Value::Absolute(width + 20.0),
                height: Value::Absolute(height + 20.0),
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
