use std::cmp::Ordering;
use std::sync::{Arc, Mutex};
use std::time::Instant;

use macroquad::window;

use crate::data_provider::Data;
use crate::screen::{self, theme::Theme, BorderParams, Letter, Style, Value};

pub struct TextBoxState<'a> {
    pub letters: Vec<Letter>,
    pub index: usize,
    pub time_started: Instant,
    pub timer: Instant,
    pub started: bool,
    pub data: &'a Data,
    pub incremental_wpm: Arc<Mutex<Vec<u16>>>,
}

pub struct TextBox<'a> {
    pub style: Style,
    pub state: TextBoxState<'a>,
}

impl<'a> TextBox<'a> {
    pub fn new(style: &Style, data: &'a Data) -> TextBox<'a> {
        // TODO:remove clone
        let text = data.get_random_quote().quote.clone();

        let letters: Vec<Letter> = text
            .chars()
            .enumerate()
            .map(|(id, c)| Letter {
                letter: c,
                color: Arc::clone(&style.theme.ghost),
                id,
            })
            .collect();

        let f1 = Arc::clone(&style.font_size);
        let f2 = Arc::clone(&style.font_size);

        TextBox {
            style: Style {
                font_size: Arc::clone(&style.font_size),
                border: Some(BorderParams {
                    size: 2.0,
                    color: Arc::clone(&style.theme.ghost),
                }),
                theme: Theme {
                    bg: Arc::clone(&style.theme.bg),
                    ghost: Arc::clone(&style.theme.ghost),
                    text: Arc::clone(&style.theme.text),
                    error: Arc::clone(&style.theme.error),
                },
                x: Value::Relative(Box::new(|| (0.5 * window::screen_width()) / 2.0)),
                y: Value::Relative(Box::new(move || {
                    let f1 = *f1.lock().unwrap();
                    (window::screen_height() - f1 * 3.0) / 2.0
                })),
                width: Value::Relative(Box::new(|| window::screen_width() / 2.0)),
                height: Value::Relative(Box::new(move || {
                    let f2 = *f2.lock().unwrap();
                    f2 * 3.0
                })),
                clip: true,
                offset_y: None,
                offset_x: None,
                padding_x: None,
                padding_y: None,
            },
            state: TextBoxState {
                letters,
                index: 0,
                time_started: Instant::now(),
                timer: Instant::now(),
                started: false,
                incremental_wpm: Arc::new(Mutex::new(vec![])),
                data,
            },
        }
    }

    pub fn refresh(&mut self) {
        let text = self.state.data.get_random_quote().quote.clone();
        let letters: Vec<Letter> = text
            .chars()
            .enumerate()
            .map(|(id, c)| Letter {
                letter: c,
                color: Arc::clone(&self.style.theme.ghost),
                id,
            })
            .collect();

        self.state.letters = letters;
        self.state.index = 0;
        self.state.started = false;

        self.style.offset_y = None;
    }

    pub fn on_type(&mut self, c: char) -> bool {
        if self.state.index == self.state.letters.len() - 1 {
            return true;
        }

        if c == self.state.letters[self.state.index].letter {
            self.state.letters[self.state.index] = Letter {
                color: Arc::clone(&self.style.theme.text),
                ..self.state.letters[self.state.index]
            };
        } else {
            self.state.letters[self.state.index] = Letter {
                color: Arc::clone(&self.style.theme.error),
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
            color: Arc::clone(&self.style.theme.ghost),
            ..self.state.letters[self.state.index]
        };
    }

    fn update_position(&mut self, line_breaks: &[usize]) {
        let mut left: i32 = 0;
        let mut right: i32 = line_breaks.len() as i32 - 1;

        while left < right {
            let mid = (left + right) / 2;

            match self.state.index.cmp(&line_breaks[mid as usize]) {
                Ordering::Less => right = mid - 1,
                Ordering::Greater => left = mid + 1,
                Ordering::Equal => {
                    left = mid;
                    break;
                }
            }
        }
        if left > 0 && self.state.index < line_breaks[left as usize] {
            left -= 1;
        }
        self.style.offset_y = Some(Value::Absolute(
            -(left as f32 * *self.style.font_size.lock().unwrap()),
        ));
    }

    pub fn get_incremental_wpm(&mut self) {
        let t = self.state.timer.elapsed().as_millis();
        if !self.state.started || t < 500 {
            return;
        }

        let v = Arc::clone(&self.state.incremental_wpm);

        //thread::spawn(move || {
        //    let mut wrongs = 0.0;
        //    let mut is_word_wrong = false;
        //
        //    for letter in &self.state.letters {
        //        if *letter.color.lock().unwrap() == *self.style.theme.error.lock().unwrap()
        //            && !is_word_wrong
        //        {
        //            wrongs += 1.0;
        //            is_word_wrong = true;
        //        }
        //        if letter.letter == ' ' {
        //            is_word_wrong = false;
        //        }
        //    }
        //
        //    let w = 1000 * 60 * (self.state.letters.len() as f32 / 5.0 - wrongs) as u128 / t;
        //
        //    let v = v.lock().unwrap();
        //});

        self.state.timer = Instant::now();
    }

    pub fn get_wpm(&self) -> u16 {
        let time_passed: u128 = self.state.time_started.elapsed().as_millis();
        let mut wrongs = 0.0;
        let mut is_word_wrong = false;

        for letter in &self.state.letters {
            let letter_color = *letter.color.lock().unwrap();
            let error_color = *self.style.theme.error.lock().unwrap();

            if letter_color == error_color && !is_word_wrong {
                wrongs += 1.0;
                is_word_wrong = true;
            }
            if letter.letter == ' ' {
                is_word_wrong = false;
            }
        }

        (1000 * 60 * (self.state.letters.len() as f32 / 5.0 - wrongs) as u128 / time_passed) as u16
    }

    pub fn update(&mut self) {
        self.style.draw_bg();

        let line_breaks =
            screen::text::print_letters_wrap(&self.style, &self.state.letters, self.state.index);
        self.update_position(&line_breaks);

        self.style.draw_mask();

        if self.state.index > 0 && !self.state.started {
            self.state.started = true;
            self.state.time_started = Instant::now();
            self.state.timer = Instant::now();
        }
    }
}
