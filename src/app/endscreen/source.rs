use std::rc::Rc;

use macroquad::text::Font;
use macroquad::window;

use crate::app::style::Style;
use crate::app::text::{PrintOptions, WrappedText};
use crate::app::theme::Theme;
use crate::app::{self, Value};

pub struct Source {
    style: Style,
    text: String,
    font: Rc<Font>,
}

impl Source {
    pub fn new(style: &Style, text: String, font: Rc<Font>) -> Self {
        let f1 = Rc::clone(&style.font_size);
        let font1 = Rc::clone(&font);

        Source {
            font,
            text: text.clone(),
            style: Style {
                theme: Theme {
                    bg: Rc::clone(&style.theme.bg),
                    text: Rc::clone(&style.theme.ghost),
                    ghost: Rc::clone(&style.theme.ghost),
                    error: Rc::clone(&style.theme.error),
                },
                font_size: Rc::clone(&style.font_size),
                width: Value::Relative(Box::new(move |_| window::screen_width())),
                x: Value::Absolute(0.0),
                y: Value::Relative(Box::new(move |_| {
                    let wt = WrappedText::new(
                        &text[..],
                        window::screen_width() - 40.0,
                        *f1.borrow(),
                        1.0,
                        Rc::clone(&font1),
                    );

                    window::screen_height() - wt.height() - 40.0
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
            PrintOptions {
                font: Some(Rc::clone(&self.font)),
                ..PrintOptions::default()
            },
        );
    }
}
