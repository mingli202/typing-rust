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
use self::component::tracker::Tracker;
use self::component::{next_button, quit_button};
use self::component::{restart_button::RestartButton, textbox::TextBox, wpm::Wmp};
use component::{Component, Style};
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
    state: Rc<RefCell<State>>,
    data: Data,
    buttons: HashMap<&'static str, Vec<Box<dyn Component>>>,
    focus: Rc<RefCell<i32>>,
}

impl Screen {
    pub fn new(data: Data) -> Self {
        Screen {
            data,
            state: Rc::new(RefCell::new(State::Typing(Mode::default()))),
            buttons: HashMap::from([("typing", vec![]), ("endscreen", vec![])]),
            focus: Rc::new(RefCell::new(-1)),
            style: Style {
                font_size: 30.0,
                ..Style::default()
            },
        }

        // TODO: animation library use threads to mutate value over time (maybe)
    }

    fn get_state(&self) -> &str {
        match *self.state.borrow() {
            State::Typing(_) => "typing",
            State::EndScreen => "endscreen",
        }
    }

    pub async fn main_loop(&mut self) -> Result<(), Box<dyn Error>> {
        let current_text = self.data.get_random_quote().quote.clone();

        let typingbox: Rc<RefCell<TextBox>> = Rc::new(RefCell::new(TextBox::new(
            current_text,
            &self.style,
            Rc::clone(&self.focus),
        )));

        let mut tracker = Tracker::new(&self.style, Rc::clone(&typingbox));

        self.buttons.entry("typing").and_modify(|v| {
            v.push(Box::new(RestartButton::new(
                &self.style,
                Rc::clone(&self.focus),
                Rc::clone(&typingbox),
            )))
        });

        self.buttons.entry("endscreen").and_modify(|v| {
            v.append(&mut vec![
                Box::new(next_button::NextButton::new(
                    &self.style,
                    Rc::clone(&self.focus),
                    Rc::clone(&typingbox),
                )),
                Box::new(quit_button::QuitButton::new(
                    &self.style,
                    Rc::clone(&self.focus),
                )),
            ])
        });

        let mut wmp = Wmp::new(&self.style, 0);

        loop {
            if let Some(k) = input::get_last_key_pressed() {
                match k {
                    KeyCode::Enter => {
                        input::clear_input_queue();
                        let current = *self.focus.borrow();
                        if current >= 0 {
                            self.buttons.get(self.get_state()).unwrap()[current as usize]
                                .click(self);
                        }
                    }
                    KeyCode::Tab => {
                        input::clear_input_queue();
                        let len = self.buttons.get(self.get_state()).unwrap().len() as i32;
                        if len > 0 {
                            let next = (*self.focus.borrow() + 1) % len;
                            *self.focus.borrow_mut() = next;
                        }
                    }
                    KeyCode::Backspace => {
                        input::clear_input_queue();
                        if let State::Typing(_) = *self.state.borrow() {
                            typingbox.borrow_mut().delete_char();
                        }
                    }
                    // this passes the keytrokes to type
                    _ => {
                        if let Some(c) = input::get_char_pressed() {
                            if self.get_state() == "typing" {
                                *self.focus.borrow_mut() = -1;
                                if typingbox.borrow_mut().ontype(c) {
                                    typingbox.borrow_mut().state.started = false;
                                    wmp = Wmp::new(&self.style, typingbox.borrow().get_wpm());
                                    *self.state.borrow_mut() = State::EndScreen;
                                }
                            } else {
                                match c {
                                    'q' => break,
                                    'n' => {
                                        self.buttons.get(self.get_state()).unwrap()[0].click(self);
                                    }
                                    _ => (),
                                }
                            }
                        }
                    }
                }
            }

            window::clear_background(*self.style.theme.bg.borrow());

            match *self.state.borrow() {
                State::Typing(_) => {
                    typingbox.borrow_mut().update();
                    tracker.update();

                    self.buttons.entry("typing").and_modify(|comps| {
                        for comp in comps {
                            comp.update();
                        }
                    });
                }
                State::EndScreen => {
                    // TODO: speed graph like in monkeytype.
                    wmp.update();

                    self.buttons.entry("endscreen").and_modify(|comps| {
                        for comp in comps {
                            comp.update();
                        }
                    });
                }
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
    pub id: usize,
}
