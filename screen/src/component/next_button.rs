use std::cell::RefCell;
use std::rc::Rc;

use macroquad::{text, window};

use crate::theme::Theme;
use crate::{Mode, Screen, State};

use super::textbox::TextBox;
use super::{BorderParams, Component, Style, Value};

pub struct NextButtonState {
    pub text: String,
    pub focus: Rc<RefCell<i32>>,
    pub id: i32,
}

pub struct NextButton {
    pub style: Style,
    pub state: NextButtonState,
    pub typingbox_ref: Rc<RefCell<TextBox>>,
}

impl Component for NextButton {
    fn update(&mut self) {
        crate::text::print_text(
            &self.style,
            &self.state.text,
            self.style.x.get(),
            self.style.y.get(),
        );

        if *self.state.focus.borrow() == self.state.id {
            self.style.draw_border();
        }
    }

    fn click(&self, screen: &Screen) {
        *self.typingbox_ref.borrow_mut() = TextBox::new(
            screen.data.get_random_quote().quote.clone(),
            &screen.style,
            Rc::clone(&screen.focus),
        );
        *screen.focus.borrow_mut() = -1;
        *screen.state.borrow_mut() = State::Typing(Mode::Quote);
    }
}

impl NextButton {
    pub fn new(
        style: &Style,
        focus: Rc<RefCell<i32>>,
        typingbox_ref: Rc<RefCell<TextBox>>,
    ) -> NextButton {
        let text = "Next (n)".to_string();

        let dim = text::measure_text(&text, None, style.font_size as u16, 1.0);
        let width = dim.width;
        let o_y = dim.offset_y;
        let font_size = style.font_size;

        NextButton {
            state: NextButtonState {
                text: text.to_string(),
                id: 0,
                focus: Rc::clone(&focus),
            },
            typingbox_ref,
            style: Style {
                border: Some(BorderParams {
                    size: 2.0,
                    color: Rc::clone(&style.theme.text),
                }),
                x: Value::Relative(Box::new(move || {
                    window::screen_width() / 2.0 - width - 10.0
                })),
                y: Value::Relative(Box::new(move || {
                    (window::screen_height() + font_size) / 2.0
                })),
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
                width: Value::Absolute(width + 20.0),
                height: Value::Absolute(font_size + 10.0),
                ..Style::default()
            },
        }
    }
}
