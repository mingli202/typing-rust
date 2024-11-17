use std::cell::RefCell;
use std::rc::Rc;

use macroquad::color::Color;

use super::{Component, Style};

pub struct TextBoxState {
    pub focus: Rc<RefCell<i32>>,
    pub id: i32,
    pub text: String,
    pub letters: Vec<Letter>,
}

pub struct TextBox {
    pub style: Style,
    pub state: TextBoxState,
    pub onclick: Option<Box<dyn Fn()>>,
}

impl TextBox {
    pub fn ontype(&self, c: char) {}
}

impl Component for TextBox {
    fn update(&self) {
        self.style.draw_bg();
        crate::text::print_text_wrap(&self.style, &self.state.letters);
        self.style.draw_mask();

        if *self.state.focus.borrow() == self.state.id {
            self.style.draw_border();
        }
    }
    fn click(&self) {
        if let Some(f) = &self.onclick {
            f();
        }
    }
}

pub struct Letter {
    pub letter: char,
    pub color: Rc<RefCell<Color>>,
}
