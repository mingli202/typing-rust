use std::cmp::Ordering;
use std::rc::Rc;
use std::time::Instant;

use macroquad::window;

use crate::app::{self, theme::Theme, BorderParams, Letter, Style, Value};

pub struct TextBoxState {
    pub letters: Vec<Letter>,
    pub index: usize,
    pub time_started: Instant,
    pub started: bool,
    pub incemental_wpm: Vec<u16>,
    pub timer: Instant,
}

pub struct TextBox {
    pub style: Style,
    pub state: TextBoxState,
}

impl TextBox {
    pub fn new(style: &Style, text: &str) -> TextBox {
        let letters: Vec<Letter> = text
            .chars()
            .enumerate()
            .map(|(id, c)| Letter {
                letter: c,
                color: Rc::clone(&style.theme.ghost),
                id,
            })
            .collect();

        let f1 = Rc::clone(&style.font_size);
        let f2 = Rc::clone(&style.font_size);

        TextBox {
            style: Style {
                font_size: Rc::clone(&style.font_size),
                border: Some(BorderParams {
                    size: 2.0,
                    color: Rc::clone(&style.theme.ghost),
                }),
                theme: Theme {
                    bg: Rc::clone(&style.theme.bg),
                    ghost: Rc::clone(&style.theme.ghost),
                    text: Rc::clone(&style.theme.text),
                    error: Rc::clone(&style.theme.error),
                },
                x: Value::Relative(Box::new(|| (0.5 * window::screen_width()) / 2.0)),
                y: Value::Relative(Box::new(move || {
                    (window::screen_height() - *f1.borrow() * 3.0) / 2.0
                })),
                width: Value::Relative(Box::new(|| window::screen_width() / 2.0)),
                height: Value::Relative(Box::new(move || *f2.borrow() * 3.0)),
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
                started: false,
                incemental_wpm: vec![],
                timer: Instant::now(),
            },
        }
    }

    pub fn refresh(&mut self, text: &str) {
        let letters: Vec<Letter> = text
            .chars()
            .enumerate()
            .map(|(id, c)| Letter {
                letter: c,
                color: Rc::clone(&self.style.theme.ghost),
                id,
            })
            .collect();

        self.state.letters = letters;
        self.state.index = 0;
        self.state.started = false;
        self.state.incemental_wpm = vec![];

        self.style.offset_y = None;
    }

    pub fn on_type(&mut self, c: char) -> bool {
        if self.state.index == self.state.letters.len() - 1 {
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
            -(left as f32 * *self.style.font_size.borrow()),
        ));
    }

    pub fn get_wpm(&self, end: Option<usize>) -> u16 {
        // thread::sleep(Duration::from_secs(1));
        let end = end.unwrap_or(self.state.letters.len());

        let time_passed: u128 = self.state.time_started.elapsed().as_millis();

        let mut wrongs = 0.0;
        let mut is_word_wrong = false;

        for i in 0..end {
            let letter = &self.state.letters[i];

            if *letter.color.borrow() == *self.style.theme.error.borrow() && !is_word_wrong {
                wrongs += 1.0;
                is_word_wrong = true;
            }
            if letter.letter == ' ' {
                is_word_wrong = false;
            }
        }

        (1000 * 60 * (end as f32 / 5.0 - wrongs) as u128 / time_passed) as u16
    }

    pub fn get_incremental_wpm(&mut self) {
        let t = self.state.timer.elapsed();
        if !self.state.started || t.as_millis() < 500 {
            return;
        }
        self.state.timer = Instant::now();

        self.state
            .incemental_wpm
            .push(self.get_wpm(Some(self.state.index)));
    }

    pub fn update(&mut self) {
        self.style.draw_bg();

        let line_breaks =
            app::text::print_letters_wrap(&self.style, &self.state.letters, self.state.index);
        self.update_position(&line_breaks);

        self.style.draw_mask();

        if self.state.index > 0 && !self.state.started {
            self.state.started = true;
            self.state.time_started = Instant::now();
            self.state.timer = Instant::now();
        }
        self.get_incremental_wpm();
    }
}
