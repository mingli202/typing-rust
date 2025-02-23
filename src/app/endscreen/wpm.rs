use std::rc::Rc;

use macroquad::text::{self, Font};
use macroquad::window;

use crate::app::text::PrintOptions;
use crate::app::{theme::Theme, Style, Value};

pub struct Wpm {
    pub wpm: String,
    pub style: Style,
}

impl Wpm {
    pub fn new(style: &Style, wmp: u16) -> Wpm {
        let f1 = Rc::clone(&style.font_size);

        Wpm {
            wpm: format!("WPM: {}", wmp),
            style: Style {
                x: Value::Relative(Box::new(move |_| {
                    (window::screen_width()
                        - text::measure_text(
                            &format!("WPM: {}", wmp),
                            None,
                            *f1.borrow() as u16,
                            1.0,
                        )
                        .width)
                        / 2.0
                })),
                y: Value::Relative(Box::new(move |_| window::screen_height() / 2.0)),
                font_size: Rc::clone(&style.font_size),
                theme: Theme {
                    bg: Rc::clone(&style.theme.bg),
                    ghost: Rc::clone(&style.theme.ghost),
                    text: Rc::clone(&style.theme.text),
                    error: Rc::clone(&style.theme.error),
                },
                ..Style::default()
            },
        }
    }

    pub fn update(&self, font: Rc<Font>) {
        crate::app::text::print_text(
            &self.style,
            &self.wpm,
            PrintOptions {
                font: Some(Rc::clone(&font)),
                ..PrintOptions::default()
            },
        );
    }
}
