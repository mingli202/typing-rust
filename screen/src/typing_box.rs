use std::cell::RefCell;
use std::rc::Rc;

use macroquad::window;

use crate::component::{BorderParams, Style, TextBox, Value};
use crate::theme::Theme;

pub fn typing_box(initial: &Style, current: Rc<RefCell<i32>>) -> Box<TextBox> {
    Box::new(TextBox::new(
        "This is a very long text that I wish to wrap and test that it works. In other words, this is but a humble placeholder for what is yet to be implemented the greatest typing speed test written in rust!".to_string(),
        Style {
            font_size: initial.font_size,
            border: Some(BorderParams {
                size: 2.0,
                color: Rc::clone(&initial.theme.ghost),
            }),
            theme: Theme {
                bg: Rc::clone(&initial.theme.bg),
                ghost: Rc::clone(&initial.theme.ghost),
                text: Rc::clone(&initial.theme.ghost),
                error: Rc::clone(&initial.theme.error),
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
        0,
        Rc::clone(&current),
        None
    ))
}
