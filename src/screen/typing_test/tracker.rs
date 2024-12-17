use std::rc::Rc;

use macroquad::window;

use crate::screen::{text, theme::Theme, Style, Value};

pub struct Tracker {
    style: Style,
}

impl Tracker {
    pub fn new(style: &Style) -> Tracker {
        let font_size = Rc::clone(&style.font_size);

        Tracker {
            style: Style {
                x: Value::Relative(Box::new(|| (0.5 * window::screen_width()) / 2.0)),
                y: Value::Relative(Box::new(move || {
                    (window::screen_height() - *font_size.borrow() * 3.0) / 2.0
                        - *font_size.borrow()
                })),
                theme: Theme {
                    bg: Rc::clone(&style.theme.bg),
                    ghost: Rc::clone(&style.theme.ghost),
                    text: Rc::clone(&style.theme.ghost),
                    error: Rc::clone(&style.theme.error),
                },
                font_size: Rc::clone(&style.font_size),
                ..Style::default()
            },
        }
    }

    pub fn update(&self, index: usize, len: usize) {
        text::print_text(
            &self.style,
            &format!("{}/{}", index, len),
            self.style.x.get(),
            self.style.y.get(),
        );
    }
}
