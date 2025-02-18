use std::rc::Rc;

use macroquad::{text, window};

use crate::app::{self, theme::Theme, BorderParams, Style, Value};

pub struct QuitButton {
    pub style: Style,
    pub text: String,
}

impl QuitButton {
    pub fn new(style: &Style) -> QuitButton {
        let text = "Quit (q)".to_string();
        let t = text.clone();
        let t1 = text.clone();
        let t2 = text.clone();

        let font_size = Rc::clone(&style.font_size);
        let f1 = Rc::clone(&style.font_size);
        let f3 = Rc::clone(&style.font_size);
        let f4 = Rc::clone(&style.font_size);

        QuitButton {
            text: text.to_string(),
            style: Style {
                border: Some(BorderParams {
                    size: 2.0,
                    color: Rc::clone(&style.theme.text),
                }),
                x: Value::Relative(Box::new(move || {
                    (window::screen_width()
                        - text::measure_text(&t, None, *f1.borrow() as u16, 1.0).width
                        - 20.0)
                        / 2.0
                })),
                y: Value::Relative(Box::new(move || {
                    (window::screen_height() / 2.0) + 2.0 * *font_size.borrow()
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
                width: Value::Relative(Box::new(move || {
                    text::measure_text(&t1, None, *f3.borrow() as u16, 1.0).width + 20.0
                })),
                height: Value::Relative(Box::new(move || {
                    text::measure_text(&t2, None, *f4.borrow() as u16, 1.0).height + 20.0
                })),
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
