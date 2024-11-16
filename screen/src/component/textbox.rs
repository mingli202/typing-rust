use std::cell::RefCell;
use std::rc::Rc;

use crate::text;

use super::{Component, Style};

pub struct TextBoxState {
    pub focus: Rc<RefCell<i32>>,
    pub id: i32,
    pub text: String,
}

pub struct TextBox {
    pub style: Style,
    pub click: Option<Box<dyn Fn()>>,
    pub state: TextBoxState,
}

impl TextBox {
    pub fn new(style: Style, state: TextBoxState, click: Option<Box<dyn Fn()>>) -> TextBox {
        TextBox {
            style,
            click,
            state,
        }
    }
}

impl Component for TextBox {
    fn update(&self) {
        self.style.draw_bg();
        text::print_text_wrap(&self.style, &self.state.text);
        self.style.draw_mask();

        if *self.state.focus.borrow() == self.state.id {
            self.style.draw_border();
        }
    }
    fn onclick(&self) {
        if let Some(f) = &self.click {
            f();
        }
    }
}
