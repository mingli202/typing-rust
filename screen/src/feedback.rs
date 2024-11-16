use std::cell::RefCell;
use std::rc::Rc;

use macroquad::window;

use crate::component::{BorderParams, Component, Style, Value};
use crate::text;
use crate::theme::Theme;

struct Feedback<'a> {
    text: Rc<RefCell<Vec<&'a String>>>,
    style: Style,
    typed: String,
    current: Rc<RefCell<i32>>,
    id: i32,
}

impl<'a> Feedback<'a> {
    fn new(
        text: Rc<RefCell<Vec<&'a String>>>,
        style: &Style,
        current: Rc<RefCell<i32>>,
    ) -> Feedback<'a> {
        Feedback {
            text,
            style: Style {
                font_size: style.font_size,
                border: None,
                theme: Theme {
                    bg: Rc::clone(&style.theme.bg),
                    ghost: Rc::clone(&style.theme.ghost),
                    text: Rc::clone(&style.theme.ghost),
                    error: Rc::clone(&style.theme.error),
                },
                x: Value::Relative(Box::new(|| (0.5 * window::screen_width()) / 2.0)),
                y: Value::Relative(Box::new(|| (window::screen_height() - 100.0) / 2.0)),
                width: Value::Relative(Box::new(|| window::screen_width() / 2.0)),
                height: Value::Absolute(100.0),
                clip: true,
                offset_y: None,
                offset_x: None,
                padding_x: Some(Value::Absolute(10.0)),
                padding_y: Some(Value::Absolute(10.0)),
            },
            id: 1,
            current: Rc::clone(&current),
            typed: String::new(),
        }
    }
}

impl<'a> Component for Feedback<'a> {
    // TODO: Custom text rendering because of error vs correct typed keys
    fn update(&self) {
        self.style.draw_bg();
        text::print_text_wrap(&self.style, &self.typed);
        self.style.draw_mask();

        if self.id == *self.current.borrow() {
            self.style.draw_border();
        }
    }

    fn onclick(&self) {
        todo!()
    }
}
