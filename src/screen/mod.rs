use crate::data_provider::Data;
use macroquad::color::Color;
use macroquad::input::{KeyCode, MouseButton};
use macroquad::{input, window};
use std::cell::RefCell;
use std::collections::HashMap;
use std::error::Error;
use std::rc::Rc;
mod component;
mod theme;

use component::{
    next_button, quit_button, restart_button::RestartButton, textbox::TextBox,
    theme_button::ThemeButton, tracker::Tracker, wpm::Wmp, Component, Style,
};
mod text;
mod util;

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

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
enum TypingState {
    Waiting,
    Typing,
}

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
enum State {
    TypingTest(TypingState),
    EndScreen,
    ThemeSelect,
}

pub struct Screen {
    style: Style,
    state: Rc<RefCell<State>>,
    data: Data,
    buttons: HashMap<State, Vec<Box<dyn Component>>>,
    focus: Rc<RefCell<i32>>,
}

impl Screen {
    pub fn new(data: Data) -> Self {
        Screen {
            data,
            state: Rc::new(RefCell::new(State::TypingTest(TypingState::Waiting))),
            buttons: HashMap::from([
                (State::TypingTest(TypingState::Waiting), vec![]),
                (State::TypingTest(TypingState::Typing), vec![]),
                (State::EndScreen, vec![]),
                (State::ThemeSelect, vec![]),
            ]),
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
            State::TypingTest(_) => "typing",
            State::EndScreen => "endscreen",
            State::ThemeSelect => "theme_select",
        }
    }

    pub async fn main_loop(&mut self) -> Result<(), Box<dyn Error>> {
        let current_text = self.data.get_random_quote().quote.clone();

        let typingbox: Rc<RefCell<TextBox>> =
            Rc::new(RefCell::new(TextBox::new(current_text, &self.style)));

        let mut tracker = Tracker::new(&self.style, Rc::clone(&typingbox));

        self.buttons
            .entry(State::TypingTest(TypingState::Waiting))
            .and_modify(|v| {
                v.append(&mut vec![
                    Box::new(RestartButton::new(
                        &self.style,
                        Rc::clone(&self.focus),
                        Rc::clone(&typingbox),
                        0,
                    )),
                    Box::new(ThemeButton::new(&self.style, Rc::clone(&self.focus), 1)),
                ])
            });

        self.buttons.entry(State::EndScreen).and_modify(|v| {
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
                            self.buttons.get(&self.state.borrow()).unwrap()[current as usize]
                                .on_click(self);
                        }
                    }
                    KeyCode::Tab => {
                        input::clear_input_queue();
                        if *self.focus.borrow() < 0 {
                            *self.focus.borrow_mut() = 0;
                        } else {
                            let len = self.buttons.get(&self.state.borrow()).unwrap().len() as i32;
                            if len > 0 {
                                let next = (*self.focus.borrow() + 1) % len;
                                *self.focus.borrow_mut() = next;
                            }
                        }
                    }
                    KeyCode::Backspace => {
                        input::clear_input_queue();
                        if let State::TypingTest(_) = *self.state.borrow() {
                            typingbox.borrow_mut().delete_char();
                        }
                    }
                    // this passes the keytrokes to type
                    _ => {
                        if let Some(c) = input::get_char_pressed() {
                            if self.get_state() == "typing" {
                                *self.focus.borrow_mut() = -2;
                                if typingbox.borrow_mut().ontype(c) {
                                    typingbox.borrow_mut().state.started = false;
                                    wmp = Wmp::new(&self.style, typingbox.borrow().get_wpm());
                                    *self.state.borrow_mut() = State::EndScreen;
                                }
                            } else {
                                match c {
                                    'q' => break,
                                    'n' => {
                                        self.buttons.get(&self.state.borrow()).unwrap()[0]
                                            .on_click(self);
                                    }
                                    _ => (),
                                }
                            }
                        }
                    }
                }
            }

            if input::is_mouse_button_pressed(MouseButton::Left) {
                let current = *self.focus.borrow();
                if current >= 0 {
                    self.buttons.get(&self.state.borrow()).unwrap()[current as usize]
                        .on_click(self);
                }
            }

            window::clear_background(*self.style.theme.bg.borrow());

            match &self.state.borrow() {
                State::TypingTest(typing_state) => {
                    typingbox.borrow_mut().update();
                    tracker.update();

                    self.buttons
                        .entry(State::TypingTest(typing_state))
                        .and_modify(|comps| {
                            for comp in comps {
                                comp.update();
                            }
                        });
                }
                State::EndScreen => {
                    // TODO: speed graph like in monkeytype.
                    wmp.update();

                    self.buttons.entry(State::EndScreen).and_modify(|comps| {
                        for comp in comps {
                            comp.update();
                        }
                    });
                }
                State::ThemeSelect => {
                    self.buttons.entry(State::ThemeSelect).and_modify(|comps| {
                        for comp in comps {
                            comp.update();
                        }
                    });
                }
                _ => (),
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
