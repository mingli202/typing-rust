use macroquad::{input, window};
use std::cell::RefCell;
use std::collections::HashMap;
use std::error::Error;
use std::rc::Rc;

use data_provider::Data;

mod theme;
use theme::Theme;

mod component;
use component::TextBox;
use component::{Component, Style, Value};

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
    font_size: f32,
    theme: Theme,
    mode: State,
    data: Data,
    components: HashMap<State, Vec<Box<dyn Component>>>,
    current: Rc<RefCell<i32>>,
}

impl Screen {
    pub fn new(
        data: Data,
        theme: Option<Theme>,
        mode: Option<Mode>,
        font_size: Option<f32>,
    ) -> Self {
        let mut initial = Screen {
            data,
            theme: theme.unwrap_or_default(),
            mode: State::Typing(mode.unwrap_or_default()),
            font_size: font_size.unwrap_or(20.0),
            components: HashMap::new(),
            current: Rc::new(RefCell::new(0)),
        };

        let typing_box = Box::new(TextBox::new(
            "This is a very long text that I wish to wrap and test that it works. In other words, this is but a humble placeholder for what is yet to be implemented the greatest typing speed test written in rust!".to_string(),
            Style {
                font_size: initial.font_size,
                border_size: Some(2.0),
                border_color: Rc::clone(&initial.theme.ghost),
                x: Value::Relative(Box::new(|| (0.5 * window::screen_width()) / 2.0)),
                y: Value::Relative(Box::new(|| (window::screen_height() - 100.0) / 2.0)),
                width: Value::Relative(Box::new(|| window::screen_width() / 2.0)),
                height: Value::Absolute(100.0),
                theme: Theme {
                    bg: Rc::clone(&initial.theme.bg),
                    ghost: Rc::clone(&initial.theme.ghost),
                    text: Rc::clone(&initial.theme.ghost),
                    error: Rc::clone(&initial.theme.error),
                },
                clip: true,
                offset_y: None,
                offset_x: None,
                padding_x: Some(Value::Absolute(10.0)),
                padding_y: Some(Value::Absolute(10.0)),
            }, 0, Rc::clone(&initial.current) 
        ));

        let r1 = Box::new(TextBox::new("hello world this is another box".to_string(), Style {
            font_size: initial.font_size,
            border_size: Some(2.0),
            border_color: Rc::clone(&initial.theme.text),
            x: Value::Absolute(0.0),
            y: Value::Absolute(0.0),
            width: Value::Absolute(100.0),
            height: Value::Absolute(100.0),
            clip: false,
            offset_x: None,
            offset_y: None,
            padding_x: None,
            padding_y: None,
            theme: Theme {
                    bg: Rc::clone(&initial.theme.bg),
                    ghost: Rc::clone(&initial.theme.ghost),
                    text: Rc::clone(&initial.theme.ghost),
                    error: Rc::clone(&initial.theme.error),
            }
        }, 1, Rc::clone(&initial.current)));

        let r2 = Box::new(TextBox::new("hello world this is yet another box".to_string(), Style {
            font_size: initial.font_size,
            border_size: Some(2.0),
            border_color: Rc::clone(&initial.theme.text),
            x: Value::Absolute(0.0),
            y: Value::Absolute(100.0),
            width: Value::Absolute(100.0),
            height: Value::Absolute(100.0),
            clip: false,
            offset_x: None,
            offset_y: None,
            padding_x: None,
            padding_y: None,
            theme: Theme {
                    bg: Rc::clone(&initial.theme.bg),
                    ghost: Rc::clone(&initial.theme.ghost),
                    text: Rc::clone(&initial.theme.ghost),
                    error: Rc::clone(&initial.theme.error),
            }
        }, 2, Rc::clone(&initial.current)));

        initial.components.entry(initial.mode.clone()).or_insert(vec![typing_box, r1, r2]);


        // TODO: feedback keep track of the first line and overwrite the ghost text
        // animation library use threads to mutate value over time (maybe)

        initial
    }

    pub async fn main_loop(&mut self) -> Result<(), Box<dyn Error>> {
        loop {
            if let Some(c) = input::get_char_pressed() {
                match c {
                    'a' => self.theme.set_atom(),
                    'c' => self.theme.set_catppuccin(),
                    '\u{0009}' => {
                        let next = (*self.current.borrow() + 1) % self.components.get(&self.mode).unwrap().len() as i32;
                        *self.current.borrow_mut() = next;
                    }, 
                    _ => (),
                }
            }

            window::clear_background(*self.theme.bg.borrow());

            for comp in self.components.get(&self.mode).unwrap() {
                comp.update();
            }

            window::next_frame().await;
        }
    }
}
