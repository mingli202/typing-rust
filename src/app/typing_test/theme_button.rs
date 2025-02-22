use std::rc::Rc;

use macroquad::{text, window};

use crate::app::{self, theme::Theme, BorderParams, Style, Value};

pub struct ThemeButton {
    pub style: Style,
    pub text: String,
}

impl ThemeButton {
    pub fn new(style: &Style) -> Self {
        let text = "Theme".to_string();

        let font_size = Rc::clone(&style.font_size);
        let f1 = Rc::clone(&style.font_size);
        let f2 = Rc::clone(&style.font_size);
        let f3 = Rc::clone(&style.font_size);

        ThemeButton {
            text,
            style: Style {
                border: Some(BorderParams {
                    size: 2.0,
                    color: Rc::clone(&style.theme.text),
                }),
                x: Value::Relative(Box::new(move |_| {
                    (window::screen_width()
                        - text::measure_text("Theme", None, *f1.borrow() as u16, 1.0).width
                        - 20.0)
                        / 2.0
                })),
                y: Value::Relative(Box::new(move |_| {
                    (window::screen_height() - *font_size.borrow() * 3.0) / 2.0
                        - 10.0
                        - 3.0 * *font_size.borrow()
                })),
                width: Value::Relative(Box::new(move |_| {
                    text::measure_text("Theme", None, *f2.borrow() as u16, 1.0).width + 20.0
                })),
                height: Value::Relative(Box::new(move |_| {
                    text::measure_text("Theme", None, *f3.borrow() as u16, 1.0).height + 20.0
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
            self.style.x.get(&self.style),
            self.style.y.get(&self.style),
        );
    }
}
