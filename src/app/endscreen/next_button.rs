use std::rc::Rc;

use macroquad::{text, window};

use crate::app::{theme::Theme, BorderParams, Style, Value};

pub struct NextButton {
    pub text: String,
    pub style: Style,
}

impl NextButton {
    pub fn new(style: &Style) -> NextButton {
        let text = "Next (n)".to_string();

        let f1 = Rc::clone(&style.font_size);
        let f2 = Rc::clone(&style.font_size);
        let f3 = Rc::clone(&style.font_size);
        let f4 = Rc::clone(&style.font_size);

        NextButton {
            text: text.to_string(),
            style: Style {
                border: Some(BorderParams {
                    size: 2.0,
                    color: Rc::clone(&style.theme.text),
                }),
                x: Value::Relative(Box::new(move |_| {
                    (window::screen_width() - *f1.borrow()) / 2.0
                        - text::measure_text(&text, None, *f1.borrow() as u16, 1.0).width
                        - 20.0
                })),
                y: Value::Relative(Box::new(move |_| {
                    (window::screen_height() + *f2.borrow()) / 2.0
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
                width: Value::Relative(Box::new(move |_| {
                    text::measure_text("Next (n)", None, *f3.borrow() as u16, 1.0).width + 20.0
                })),
                height: Value::Relative(Box::new(move |_| {
                    text::measure_text("Next (n)", None, *f4.borrow() as u16, 1.0).height + 20.0
                })),
                ..Style::default()
            },
        }
    }

    pub fn update(&self) {
        crate::app::text::print_text(
            &self.style,
            &self.text,
            self.style.x.get(),
            self.style.y.get(),
        );
    }
}
