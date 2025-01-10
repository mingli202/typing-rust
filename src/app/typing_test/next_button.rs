use std::rc::Rc;

use macroquad::{text, window};

use crate::app::{self, theme::Theme};

use crate::app::{BorderParams, Style, Value};

pub struct NextButton {
    pub style: Style,
    pub text: String,
}

impl NextButton {
    pub fn new(style: &Style) -> NextButton {
        let font_size = Rc::clone(&style.font_size);
        let f = Rc::clone(&style.font_size);
        let f1 = Rc::clone(&style.font_size);
        let f2 = Rc::clone(&style.font_size);
        let f3 = Rc::clone(&style.font_size);

        NextButton {
            text: "Next".to_string(),
            style: Style {
                border: Some(BorderParams {
                    size: 2.0,
                    color: Rc::clone(&style.theme.text),
                }),
                x: Value::Relative(Box::new(move || {
                    window::screen_width() / 2.0
                        - text::measure_text("Next", None, *f1.borrow() as u16, 1.0).width
                        - 20.0
                        - *f.borrow() / 2.0
                })),
                y: Value::Relative(Box::new(move || {
                    (window::screen_height() + *font_size.borrow() * 3.0) / 2.0
                        + *font_size.borrow()
                })),
                width: Value::Relative(Box::new(move || {
                    text::measure_text("Next", None, *f2.borrow() as u16, 1.0).width + 20.0
                })),
                height: Value::Relative(Box::new(move || {
                    text::measure_text("Next", None, *f3.borrow() as u16, 1.0).height + 20.0
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
