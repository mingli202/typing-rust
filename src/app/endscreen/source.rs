use std::rc::Rc;

use macroquad::{text, window};

use crate::app::style::Style;
use crate::app::theme::Theme;
use crate::app::{self, Value};

pub struct Source {
    style: Style,
    text: String,
}

impl Source {
    pub fn new(text: String, style: &Style) -> Self {
        let f1 = Rc::clone(&style.font_size);

        Source {
            text: text.clone(),
            style: Style {
                theme: Theme {
                    bg: Rc::clone(&style.theme.bg),
                    text: Rc::clone(&style.theme.ghost),
                    ghost: Rc::clone(&style.theme.ghost),
                    error: Rc::clone(&style.theme.error),
                },
                font_size: Rc::clone(&style.font_size),
                x: Value::Relative(Box::new(move || {
                    window::screen_width()
                        - text::measure_text(&text[..], None, *f1.borrow() as u16, 1.0).width
                        - 40.0
                })),
                y: Value::Relative(Box::new(move || window::screen_height() - 30.0 - 40.0)),
                padding_x: Some(Value::Absolute(20.0)),
                padding_y: Some(Value::Absolute(20.0)),
                ..Style::default()
            },
        }
    }

    pub fn update(&self) {
        app::text::print_text(
            &self.style,
            &self.text,
            self.style.x.get(),
            self.style.y.get(),
        );
    }
}
