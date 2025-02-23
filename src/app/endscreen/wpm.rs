use std::rc::Rc;

use macroquad::text::{self, Font};
use macroquad::window;

use crate::app::text::PrintOptions;
use crate::app::{theme::Theme, Style, Value};

pub struct Wpm {
    pub wpm: String,
    pub style: Style,
    font: Rc<Font>,
}

impl Wpm {
    pub fn new(style: &Style, wmp: u16, font: Rc<Font>) -> Wpm {
        let f1 = Rc::clone(&style.font_size);
        let f2 = Rc::clone(&style.font_size);

        let font1 = Rc::clone(&font);
        let font2 = Rc::clone(&font);

        Wpm {
            font,
            wpm: format!("WPM: {}", wmp),
            style: Style {
                x: Value::Relative(Box::new(move |_| {
                    (window::screen_width()
                        - text::measure_text(
                            &format!("WPM: {}", wmp),
                            Some(&font1),
                            *f1.borrow() as u16,
                            1.0,
                        )
                        .width)
                        / 2.0
                })),
                y: Value::Relative(Box::new(move |_| {
                    window::screen_height() / 2.0
                        - text::measure_text(
                            &format!("WPM: {}", wmp),
                            Some(&font2),
                            *f2.borrow() as u16,
                            1.0,
                        )
                        .height
                })),
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

    pub fn update(&self) {
        crate::app::text::print_text(
            &self.style,
            &self.wpm,
            PrintOptions {
                font: Some(Rc::clone(&self.font)),
                ..PrintOptions::default()
            },
        );
    }
}
