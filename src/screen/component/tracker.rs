use std::cell::RefCell;
use std::rc::Rc;

use macroquad::window;

use crate::screen::{text, theme::Theme};

use super::textbox::TextBox;
use super::{Component, Style, Value};

pub struct Tracker {
    typingbox_ref: Rc<RefCell<TextBox>>,
    style: Style,
}

impl Component for Tracker {
    fn update(&mut self) {
        let i = self.typingbox_ref.borrow().state.index;
        let len = self.typingbox_ref.borrow().state.letters.len();
        text::print_text(
            &self.style,
            &format!("{}/{}", i, len),
            self.style.x.get(),
            self.style.y.get(),
        );
    }
}

impl Tracker {
    pub fn new(style: &Style, typingbox_ref: Rc<RefCell<TextBox>>) -> Tracker {
        let font_size = style.font_size;

        Tracker {
            style: Style {
                x: Value::Relative(Box::new(|| (0.5 * window::screen_width()) / 2.0)),
                y: Value::Relative(Box::new(move || {
                    (window::screen_height() - font_size * 3.0) / 2.0 - font_size
                })),
                theme: Theme {
                    bg: Rc::clone(&style.theme.bg),
                    ghost: Rc::clone(&style.theme.ghost),
                    text: Rc::clone(&style.theme.ghost),
                    error: Rc::clone(&style.theme.error),
                },
                font_size: style.font_size,
                ..Style::default()
            },
            typingbox_ref: Rc::clone(&typingbox_ref),
        }
    }
}
