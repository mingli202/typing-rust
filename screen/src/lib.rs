use data_provider::Data;
use macroquad::input::KeyCode;
use macroquad::{input, window};
use std::cell::RefCell;
use std::collections::HashMap;
use std::error::Error;
use std::rc::Rc;
mod component;
mod theme;
use component::{Component, Style};
mod text;
mod typing_box;

#[derive(PartialEq, Hash, Eq, Clone)]
pub enum Mode {
    WordCount(usize),
    TimeSec(usize),
    Quote,
}

impl Default for Mode {
    fn default() -> Self {
        Mode::TimeSec(30)
    }
}

#[derive(Eq, Hash, PartialEq, Clone)]
enum State {
    Typing(Mode),
    EndScreen,
}

pub struct Screen {
    style: Style,
    state: State,
    data: Data,
    components: HashMap<&'static str, Vec<Box<dyn Component>>>,
    focus: Rc<RefCell<i32>>,
}

impl Screen {
    pub fn new(data: Data) -> Self {
        let mut initial = Screen {
            data,
            state: State::Typing(Mode::default()),
            components: HashMap::from([("typing", vec![]), ("endscreen", vec![])]),
            focus: Rc::new(RefCell::new(0)),
            style: Style {
                font_size: 20.0,
                ..Style::default()
            },
        };

        initial.components.entry("typing").and_modify(|v| v.append(&mut vec![
            typing_box::typing_box(
                "This is a very long text that I wish to wrap and test that it works. In other words, this is but a humble placeholder for what is yet to be implemented the greatest typing speed test written in rust!".to_string(),
                &initial.style,
                Rc::clone(&initial.focus),
            ),
        ]));

        // TODO: feedback keep track of the first line and overwrite the ghost text
        // animation library use threads to mutate value over time (maybe)
        initial
    }

    pub async fn main_loop(&mut self) -> Result<(), Box<dyn Error>> {
        let current_text = Rc::new(RefCell::new(self.data.get_n_random_words(100)));

        let typing_box = typing_box::typing_box(
                "This is a very long text that I wish to wrap and test that it works. In other words, this is but a humble placeholder for what is yet to be implemented the greatest typing speed test written in rust!".to_string(),
                &self.style,
                Rc::clone(&self.focus));

        let mut state = "typing";

        loop {
            if let Some(k) = input::get_last_key_pressed() {
                match k {
                    KeyCode::Enter => {
                        let current = *self.focus.borrow() as usize;
                        self.components.get(state).unwrap()[current].click();
                    }
                    KeyCode::Tab => {
                        let next = (*self.focus.borrow() + 1)
                            % self.components.get(state).unwrap().len() as i32;
                        *self.focus.borrow_mut() = next;
                    }
                    // this passes the keytrokes to type
                    _ => {
                        if let Some(c) = input::get_char_pressed() {
                            if let State::Typing(_) = self.state {
                                typing_box.ontype(c);
                            }
                        }
                    }
                }
            }

            window::clear_background(*self.style.theme.bg.borrow());

            for comp in self.components.get(state).unwrap() {
                comp.update();
            }

            window::next_frame().await;
        }
    }
}
