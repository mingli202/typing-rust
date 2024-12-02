use std::cell::RefCell;
use std::rc::Rc;

use macroquad::{text, window};

use crate::screen::util;
use crate::screen::{self, theme::Theme, Screen};

use super::textbox::TextBox;
use super::{BorderParams, Component, Style, Value};

pub struct NextButtonState {
    pub text: String,
    pub focus: Rc<RefCell<i32>>,
    pub id: i32,
}

pub struct RestartButton {
    pub style: Style,
    pub state: NextButtonState,
    pub typingbox_ref: Rc<RefCell<TextBox>>,
}

impl Component for RestartButton {
    fn update(&mut self) {
        screen::text::print_text(
            &self.style,
            &self.state.text,
            self.style.x.get(),
            self.style.y.get(),
        );

        if *self.state.focus.borrow() == self.state.id {
            self.style.draw_border();
        }
        util::handle_mouse_focus(&self.style, self.state.id, Rc::clone(&self.state.focus));
    }

    fn click(&self, screen: &Screen) {
        *self.typingbox_ref.borrow_mut() = TextBox::new(
            screen.data.get_random_quote().quote.clone(),
            &screen.style,
            Rc::clone(&screen.focus),
        );
        *screen.focus.borrow_mut() = -1;
    }
}

impl RestartButton {
    pub fn new(
        style: &Style,
        focus: Rc<RefCell<i32>>,
        typingbox_ref: Rc<RefCell<TextBox>>,
        id: i32,
    ) -> RestartButton {
        let text = "Restart".to_string();

        let dim = text::measure_text(&text, None, style.font_size as u16, 1.0);
        let width = dim.width;
        let o_y = dim.offset_y;
        let font_size = style.font_size;

        RestartButton {
            state: NextButtonState {
                text: "Restart".to_string(),
                id,
                focus: Rc::clone(&focus),
            },
            typingbox_ref,
            style: Style {
                border: Some(BorderParams {
                    size: 2.0,
                    color: Rc::clone(&style.theme.text),
                }),
                x: Value::Relative(Box::new(move || (window::screen_width() - width) / 2.0)),
                y: Value::Relative(Box::new(move || {
                    (window::screen_height() + font_size * 3.0 + 10.0) / 2.0 + 30.0
                })),
                width: Value::Absolute(width + 20.0),
                height: Value::Absolute(font_size + 5.0),
                font_size: style.font_size,
                theme: Theme {
                    bg: Rc::clone(&style.theme.bg),
                    ghost: Rc::clone(&style.theme.ghost),
                    text: Rc::clone(&style.theme.ghost),
                    error: Rc::clone(&style.theme.error),
                },
                padding_x: Some(Value::Absolute(10.0)),
                padding_y: Some(Value::Absolute(10.0)),
                offset_y: Some(Value::Absolute(o_y)),
                ..Style::default()
            },
        }
    }
}
