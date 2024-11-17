use std::cell::RefCell;
use std::rc::Rc;

use macroquad::window;

use crate::component::textbox::*;
use crate::component::{BorderParams, Style, Value};
use crate::theme::Theme;
use crate::Letter;

pub fn typing_box(text: String, style: &Style, focus: Rc<RefCell<i32>>) -> TextBox {
    let letters: Vec<Letter> = text
        .chars()
        .map(|c| Letter {
            letter: c,
            color: Rc::clone(&style.theme.ghost),
        })
        .collect();

    TextBox {
        style: Style {
            font_size: style.font_size,
            border: Some(BorderParams {
                size: 2.0,
                color: Rc::clone(&style.theme.ghost),
            }),
            theme: Theme {
                bg: Rc::clone(&style.theme.bg),
                ghost: Rc::clone(&style.theme.ghost),
                text: Rc::clone(&style.theme.text),
                error: Rc::clone(&style.theme.error),
            },
            x: Value::Relative(Box::new(|| (0.5 * window::screen_width()) / 2.0)),
            y: Value::Relative(Box::new(|| {
                window::screen_height() * (1.0 - 1.0 / (2.0 * 1.61)) / 2.0
            })),
            width: Value::Relative(Box::new(|| window::screen_width() / 2.0)),
            height: Value::Relative(Box::new(|| window::screen_height() / (2.0 * 1.61))),
            clip: true,
            offset_y: None,
            offset_x: None,
            padding_x: Some(Value::Absolute(10.0)),
            padding_y: Some(Value::Absolute(10.0)),
        },
        state: TextBoxState {
            focus: Rc::clone(&focus),
            id: 0,
            letters,
            index: 0,
        },
        onclick: None,
    }
}
