use std::rc::Rc;

use macroquad::text::Font;
use macroquad::{text, window};

use crate::app::text::PrintOptions;
use crate::app::theme::Theme;

use crate::app::{BorderParams, Style, Value};

pub struct RestartButton {
    pub style: Style,
    pub text: String,
    font: Rc<Font>,
}

impl RestartButton {
    pub fn new(style: &Style, font: Rc<Font>) -> RestartButton {
        let font_size = Rc::clone(&style.font_size);
        let f1 = Rc::clone(&style.font_size);
        let f2 = Rc::clone(&style.font_size);
        let f3 = Rc::clone(&style.font_size);

        let font1 = Rc::clone(&font);
        let font2 = Rc::clone(&font);

        RestartButton {
            font,
            text: "Restart".to_string(),
            style: Style {
                border: Some(BorderParams {
                    size: 2.0,
                    color: Rc::clone(&style.theme.text),
                }),
                x: Value::Relative(Box::new(move |_| {
                    window::screen_width() / 2.0 + *f1.borrow() / 2.0
                })),
                y: Value::Relative(Box::new(move |_| {
                    (window::screen_height() + *font_size.borrow() * 3.0) / 2.0
                        + *font_size.borrow()
                })),
                width: Value::Relative(Box::new(move |_| {
                    text::measure_text("Restart", Some(&font1), *f2.borrow() as u16, 1.0).width
                        + 20.0
                })),
                height: Value::Relative(Box::new(move |_| {
                    text::measure_text("Restart", Some(&font2), *f3.borrow() as u16, 1.0).height
                        + 20.0
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
        crate::app::text::print_text(
            &self.style,
            &self.text,
            PrintOptions {
                font: Some(Rc::clone(&self.font)),
                ..PrintOptions::default()
            },
        );
    }
}
