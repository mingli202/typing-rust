use macroquad::input::KeyCode;
use macroquad::{input, window};
use std::cell::RefCell;
use std::collections::HashMap;
use std::error::Error;
use std::rc::Rc;

use data_provider::Data;

mod theme;

mod component;
use component::{Component, Style};

use self::component::Button;

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

trait C: Component + Button {}

pub struct Screen {
    style: Style,
    mode: State,
    data: Data,
    components: HashMap<State, Vec<Box<dyn C>>>,
    current: Rc<RefCell<i32>>,
}

impl Screen {
    pub fn new(data: Data) -> Self {
        let mut initial = Screen {
            data,
            mode: State::Typing(Mode::default()),
            components: HashMap::new(),
            current: Rc::new(RefCell::new(0)),
            style: Style {
                font_size: 20.0,
                ..Style::default()
            },
        };

        initial
            .components
            .entry(initial.mode.clone())
            .or_insert(vec![typing_box::typing_box(
                &initial.style,
                Rc::clone(&initial.current),
            )]);

        // TODO: feedback keep track of the first line and overwrite the ghost text
        // animation library use threads to mutate value over time (maybe)

        initial
    }

    pub async fn main_loop(&mut self) -> Result<(), Box<dyn Error>> {
        loop {
            if let Some(k) = input::get_last_key_pressed() {
                match k {
                    KeyCode::Enter => {
                        let current = *self.current.borrow() as usize;
                        self.components.get(&self.mode).unwrap()[current].onclick();
                    }
                    KeyCode::Tab => {
                        let next = (*self.current.borrow() + 1)
                            % self.components.get(&self.mode).unwrap().len() as i32;
                        *self.current.borrow_mut() = next;
                    }
                    // this passes the keytrokes to type
                    _ => if let Some(c) = input::get_char_pressed() {},
                }
            }

            window::clear_background(*self.style.theme.bg.borrow());

            for comp in self.components.get(&self.mode).unwrap() {
                comp.update();
            }

            window::next_frame().await;
        }
    }
}
