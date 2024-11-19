use data_provider::Data;
use macroquad::color::Color;
use macroquad::input::KeyCode;
use macroquad::{input, window};
use std::cell::RefCell;
use std::collections::HashMap;
use std::error::Error;
use std::rc::Rc;
mod component;
mod theme;
use component::{Component, Style};

use self::component::tracker::Tracker;
use self::component::{restart_button::RestartButton, textbox::TextBox};
mod text;

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
        Screen {
            data,
            state: State::Typing(Mode::default()),
            components: HashMap::from([("typing", vec![]), ("endscreen", vec![])]),
            focus: Rc::new(RefCell::new(-1)),
            style: Style {
                font_size: 30.0,
                ..Style::default()
            },
        }

        // TODO: animation library use threads to mutate value over time (maybe)
    }

    fn get_state(&self) -> &str {
        match self.state {
            State::Typing(_) => "typing",
            State::EndScreen => "endscreen",
        }
    }

    pub async fn main_loop(&mut self) -> Result<(), Box<dyn Error>> {
        //let mut current_text = self
        //    .data
        //    .get_n_random_words(10)
        //    .iter()
        //    .fold(String::new(), |acc, el| acc + el + " ");
        //current_text.pop();

        let current_text = self.data.get_random_quote().quote.clone();

        let typingbox: Rc<RefCell<TextBox>> = Rc::new(RefCell::new(TextBox::new(
            current_text,
            &self.style,
            Rc::clone(&self.focus),
        )));

        let mut tracker = Tracker::new(&self.style, Rc::clone(&typingbox));

        let restart_button =
            RestartButton::new(&self.style, Rc::clone(&self.focus), Rc::clone(&typingbox));

        self.components
            .entry("typing")
            .and_modify(|v| v.push(Box::new(restart_button)));

        loop {
            if let Some(k) = input::get_last_key_pressed() {
                match k {
                    KeyCode::Enter => {
                        input::clear_input_queue();
                        let current = *self.focus.borrow();
                        if current >= 0 {
                            self.components.get(self.get_state()).unwrap()[current as usize]
                                .click(self);
                        }
                    }
                    KeyCode::Tab => {
                        input::clear_input_queue();
                        let len = self.components.get(self.get_state()).unwrap().len() as i32;
                        if len > 0 {
                            let next = (*self.focus.borrow() + 1) % len;
                            *self.focus.borrow_mut() = next;
                        }
                    }
                    KeyCode::Backspace => {
                        input::clear_input_queue();
                        if let State::Typing(_) = self.state {
                            typingbox.borrow_mut().delete_char();
                        }
                    }
                    // this passes the keytrokes to type
                    _ => {
                        *self.focus.borrow_mut() = -1;
                        if let Some(c) = input::get_char_pressed() {
                            if let State::Typing(_) = self.state {
                                if typingbox.borrow_mut().ontype(c) {
                                    self.state = State::EndScreen;
                                }
                            } else {
                                match c {
                                    'q' => break,
                                    'n' => {
                                        *typingbox.borrow_mut() = TextBox::new(
                                            self.data.get_random_quote().quote.clone(),
                                            &self.style,
                                            Rc::clone(&self.focus),
                                        );
                                        self.state = State::Typing(Mode::Quote);
                                    }
                                    _ => (),
                                }
                            }
                        }
                    }
                }
            }

            window::clear_background(*self.style.theme.bg.borrow());

            match self.state {
                State::Typing(_) => {
                    typingbox.borrow_mut().update();
                    tracker.update();

                    self.components.entry("typing").and_modify(|comps| {
                        for comp in comps {
                            comp.update();
                        }
                    });
                }
                State::EndScreen => {}
            }

            window::next_frame().await;
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Letter {
    pub letter: char,
    pub color: Rc<RefCell<Color>>,
}
