use std::rc::Rc;

use macroquad::{text, window};

use crate::app::style::Style;
use crate::app::text::WrappedText;
use crate::app::theme::Theme;
use crate::app::{self, Value};

pub struct Source {
    style: Style,
    text: String,
}

impl Source {
    pub fn new(text: String, style: &Style) -> Self {
        let f1 = Rc::clone(&style.font_size);
        let text_color = Rc::clone(&style.theme.text);

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
                width: Value::Relative(Box::new(move |_| window::screen_width() - 40.0)),
                x: Value::Absolute(0.0),
                y: Value::Relative(Box::new(move |_| {
                    let wt = WrappedText::new(
                        &text[..],
                        window::screen_width() - 40.0,
                        text::TextParams {
                            font_size: *f1.borrow() as u16,
                            color: *text_color.borrow(),
                            ..text::TextParams::default()
                        },
                    );

                    window::screen_height() - wt.get_height() - 40.0
                })),
                padding_x: Some(Value::Absolute(20.0)),
                padding_y: Some(Value::Absolute(20.0)),
                wrap: true,
                ..Style::default()
            },
        }
    }

    pub fn update(&self) {
        app::text::print_text(
            &self.style,
            &self.text,
            self.style.x.get(&self.style),
            self.style.y.get(&self.style),
        );
    }
}
