use std::cell::RefCell;
use std::rc::Rc;

use crate::Letter;

use super::{Component, Style};

pub struct TextBoxState {
    pub focus: Rc<RefCell<i32>>,
    pub id: i32,
    pub letters: Vec<Letter>,
    pub index: usize,
}

pub struct TextBox {
    pub style: Style,
    pub state: TextBoxState,
    pub onclick: Option<Box<dyn Fn()>>,
}

// TODO:pass in character, and update real time colors
impl TextBox {
    pub fn ontype(&mut self, c: char) -> bool {
        if self.state.index == self.state.letters.len() {
            return true;
        }

        if c == self.state.letters[self.state.index].letter {
            self.state.letters[self.state.index] = Letter {
                color: Rc::clone(&self.style.theme.text),
                ..self.state.letters[self.state.index]
            };
        } else {
            self.state.letters[self.state.index] = Letter {
                color: Rc::clone(&self.style.theme.error),
                ..self.state.letters[self.state.index]
            };
        }

        self.state.index += 1;
        false
    }
    pub fn delete_char(&mut self) {
        if self.state.index == 0 {
            return;
        }
        self.state.index -= 1;
        self.state.letters[self.state.index] = Letter {
            color: Rc::clone(&self.style.theme.ghost),
            ..self.state.letters[self.state.index]
        };
    }
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
