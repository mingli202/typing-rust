use std::rc::Rc;

use macroquad::text::{self, TextDimensions};
use macroquad::window;

use crate::screen::{theme::Theme, Style, Value};

pub struct Wpm {
    pub wpm: String,
    pub style: Style,
}

impl Wpm {
    pub fn new(style: &Style, wmp: u16) -> Wpm {
        let font_size = Rc::clone(&style.font_size);
        let f1 = Rc::clone(&style.font_size);
        let f2 = Rc::clone(&style.font_size);

        Wpm {
            wpm: format!("WPM: {}", wmp),
            style: Style {
                x: Value::Relative(Box::new(move || {
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
                y: Value::Relative(Box::new(move || {
                    let TextDimensions {
                        height, offset_y, ..
                    } = text::measure_text(
                        &format!("WPM: {}", wmp),
                        None,
                        *f2.borrow() as u16,
                        1.0,
                    );
                    (window::screen_height() - height + offset_y - *font_size.borrow()) / 2.0
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
        macroquad::text::draw_text(
            &self.wpm,
            self.style.x.get(),
            self.style.y.get(),
            *self.style.font_size.borrow(),
            *self.style.theme.text.borrow(),
        );
    }
}
