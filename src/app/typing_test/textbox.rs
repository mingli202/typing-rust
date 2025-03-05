use std::cmp::{max, Ordering};
use std::rc::Rc;
use std::time::Instant;

use macroquad::text::{Font, TextDimensions, TextParams};
use macroquad::{shapes, text, window};

use crate::app::{theme::Theme, BorderParams, Letter, Style, Value};
use crate::app::{util, Word};

pub struct TextBoxState {
    pub words: Vec<Word>,
    pub word_index: usize,
    pub char_index: usize,
    pub time_started: Instant,
    pub started: bool,
    pub wrongs: usize,
    pub char_typed: i32,
}

pub struct TextBox {
    pub style: Style,
    pub state: TextBoxState,
    pub font: Rc<Font>,
}

impl TextBox {
    pub fn new(style: &Style, text: String, font: Rc<Font>) -> TextBox {
        let words: Vec<Word> = text
            .split(" ")
            .enumerate()
            .map(|(id, word)| Word::from_str(style, word, id))
            .collect();

        /*  0        1        2       3     4      5       6        7
         * [[hello], [world], [this], [is], [the], [best], [thing], [ever]]
         *   01234    01234    0123    01    012    0123    01234    0123
         * */

        let f1 = Rc::clone(&style.font_size);
        let f2 = Rc::clone(&style.font_size);
        let f3 = Rc::clone(&style.font_size);

        let font1 = Rc::clone(&font);

        TextBox {
            font,
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
                x: Value::Relative(Box::new(move |this| {
                    (window::screen_width() - this.width()) / 2.0
                })),
                y: Value::Relative(Box::new(move |_| {
                    (window::screen_height() - *f1.borrow() * (3.0 + 2.0 * 0.15)) / 2.0
                })),
                width: Value::Relative(Box::new(move |_| {
                    util::clamp(
                        0.0,
                        window::screen_width() * 0.8,
                        text::measure_text("o", Some(&font1), *f3.borrow() as u16, 1.0).width
                            * 80.0,
                    )
                })),
                height: Value::Relative(Box::new(move |_| *f2.borrow() * (3.0 + 2.0 * 0.15))),
                clip: true,
                ..Style::default()
            },
            state: TextBoxState {
                word_index: 0,
                char_index: 0,
                time_started: Instant::now(),
                started: false,
                words,
                wrongs: 0,
                char_typed: 0,
            },
        }
    }

    pub fn refresh(&mut self, text: String) {
        let words: Vec<Word> = text
            .split(" ")
            .enumerate()
            .map(|(id, word)| Word::from_str(&self.style, word, id))
            .collect();

        self.state.words = words;
        self.state.word_index = 0;
        self.state.char_index = 0;
        self.state.started = false;
        self.state.wrongs = 0;
        self.state.char_typed = 0;
        self.style.offset_y = None;
    }

    pub fn on_type(&mut self, c: char) -> bool {
        if self.state.word_index == self.state.words.len() - 1 && c == ' ' {
            self.state.wrongs += 1;
            return true;
        }

        if c == ' ' {
            self.submit_word();
            return false;
        }

        if self.state.char_index == self.state.words[self.state.word_index].letters.len() {
            // if its the end of a word allow overflow
            self.state.words[self.state.word_index]
                .letters
                .push(Letter {
                    letter: c,
                    color: Rc::clone(&self.style.theme.error),
                    char_id: self.state.char_index,
                    word_id: self.state.word_index,
                });

            self.state.char_index += 1;
            self.state.words[self.state.word_index].last_typed = self.state.char_index;
            return false;
        }

        self.state.char_typed += 1;

        let correct = self.state.words[self.state.word_index].letters[self.state.char_index].letter;

        self.state.words[self.state.word_index].letters[self.state.char_index] = if c == correct {
            Letter {
                color: Rc::clone(&self.style.theme.text),
                ..(self.state.words[self.state.word_index].letters[self.state.char_index]).clone()
            }
        } else {
            Letter {
                color: Rc::clone(&self.style.theme.error),
                ..(self.state.words[self.state.word_index].letters[self.state.char_index]).clone()
            }
        };

        self.state.char_index += 1;
        self.state.words[self.state.word_index].last_typed = self.state.char_index;

        if self.state.word_index == self.state.words.len() - 1
            && self.state.words[self.state.word_index]
                .letters
                .iter()
                .all(|l| *l.color.borrow() == *self.style.theme.text.borrow())
        {
            self.submit_word();
            return true;
        }

        false
    }

    fn submit_word(&mut self) {
        // check if current word is wrong
        if !self.state.words[self.state.word_index]
            .letters
            .iter()
            .all(|l| *l.color.borrow() == *self.style.theme.text.borrow())
        {
            if !self.state.words[self.state.word_index].is_error {
                self.state.wrongs += 1;
                self.state.words[self.state.word_index].is_error = true;
            }
        } else if self.state.words[self.state.word_index].is_error {
            self.state.wrongs -= 1;
            self.state.words[self.state.word_index].is_error = false;
        }

        self.state.char_typed += max(
            0,
            self.state.words[self.state.word_index].word.len() as i32 + 1
                - self.state.char_index as i32,
        );

        // move to the next word
        self.state.word_index += 1;
        self.state.char_index = 0;
    }

    pub fn delete_char(&mut self) {
        if self.state.char_index == 0 {
            // return if its the first word
            if self.state.word_index == 0 {
                return;
            }

            // move back to the previous word
            self.state.word_index -= 1;
            self.state.char_index = self.state.words[self.state.word_index].last_typed;

            self.state.char_typed -= max(
                0,
                self.state.words[self.state.word_index].word.len() as i32 + 1
                    - self.state.char_index as i32,
            );

            return;
        }

        self.state.char_index -= 1;
        self.state.words[self.state.word_index].last_typed = self.state.char_index;

        // check if we are deleting overflow
        if self.state.char_index >= self.state.words[self.state.word_index].word.len() {
            self.state.words[self.state.word_index].letters.pop();
        } else {
            self.state.char_typed -= 1;
            self.state.words[self.state.word_index].letters[self.state.char_index] = Letter {
                color: Rc::clone(&self.style.theme.ghost),
                ..(self.state.words[self.state.word_index].letters[self.state.char_index]).clone()
            };
        }
    }

    fn update_position(&mut self, line_breaks: &[usize]) {
        let mut left: i32 = 0;
        let mut right: i32 = line_breaks.len() as i32 - 1;

        while left < right {
            let mid = (left + right) / 2;

            match self.state.word_index.cmp(&line_breaks[mid as usize]) {
                Ordering::Less => right = mid - 1,
                Ordering::Greater => left = mid + 1,
                Ordering::Equal => {
                    left = mid;
                    break;
                }
            }
        }
        if left > 0 && self.state.word_index < line_breaks[left as usize] {
            left -= 1;
        }
        self.style.offset_y = Some(Value::Absolute(
            -(left as f32 * *self.style.font_size.borrow() * 1.15),
        ));
    }

    pub fn get_wpm(&self) -> f32 {
        let time_passed: u128 = self.state.time_started.elapsed().as_millis();

        if time_passed == 0 {
            return 0.0;
        }

        let wpm = (1000.0 * 60.0 * (self.state.char_typed as f32 / 5.0 - self.state.wrongs as f32))
            / time_passed as f32;

        if wpm > 0.0 {
            wpm
        } else {
            0.0
        }
    }

    pub fn get_accuracey(&self) -> i32 {
        100 - 100 * 5 * self.state.wrongs as i32 / self.state.char_typed
    }

    pub fn update(&mut self) {
        self.style.draw_bg();

        let line_breaks = self.print_words();
        self.update_position(&line_breaks);

        self.style.draw_mask();

        if self.state.char_index > 0 && !self.state.started {
            self.state.started = true;
            self.state.time_started = Instant::now();
        }
    }

    fn print_words(&self) -> Vec<usize> {
        let mut line_breaks = vec![];

        let mut lines = 0;

        let mut line: Vec<&Word> = vec![];

        let p_x = match &self.style.padding_x {
            Some(p) => p.get(&self.style),
            _ => 0.0,
        };

        for word in &self.state.words {
            let l = line
                .iter()
                .map(|w| {
                    w.letters
                        .iter()
                        .fold(String::new(), |acc, l| acc + &l.letter.to_string())
                })
                .collect::<Vec<String>>()
                .join(" ")
                + " "
                + &word
                    .letters
                    .iter()
                    .fold(String::new(), |acc, l| acc + &l.letter.to_string());

            let TextDimensions { width, .. } = text::measure_text(
                &l,
                Some(&self.font),
                *self.style.font_size.borrow() as u16,
                1.0,
            );

            if width > self.style.width.get(&self.style) - 2.0 * p_x {
                let y = *self.style.font_size.borrow() * lines as f32 * 1.15;

                self.print_letters(
                    &line,
                    self.style.x.get(&self.style),
                    self.style.y.get(&self.style) + y,
                    self.state.word_index,
                    self.state.char_index,
                );

                line_breaks.push(word.id);
                lines += 1;
                line = vec![word];
            } else {
                line.push(word);
            }
        }

        let y = *self.style.font_size.borrow() * lines as f32 * 1.15;
        self.print_letters(
            &line,
            self.style.x.get(&self.style),
            self.style.y.get(&self.style) + y,
            self.state.word_index,
            self.state.char_index,
        );

        line_breaks
    }

    fn print_letters(&self, line: &[&Word], x: f32, y: f32, word_index: usize, char_index: usize) {
        let mut letters: Vec<Letter> = vec![];

        for word in line {
            for letter in &word.letters {
                letters.push(letter.clone());
            }
            letters.push(Letter {
                letter: ' ',
                color: Rc::clone(&self.style.theme.ghost),
                char_id: word.letters.len(),
                word_id: word.id,
            })
        }

        letters.pop();

        let mut offset_x = 0.0;
        let offset_y = text::measure_text(
            &letters
                .iter()
                .fold(String::new(), |acc, l| acc + &l.letter.to_string()),
            Some(&self.font),
            *self.style.font_size.borrow() as u16,
            1.0,
        )
        .offset_y;

        for letter in &letters {
            offset_x += self
                .print_letter(letter, x + offset_x, y + offset_y, word_index, char_index)
                .width;
        }
    }

    fn print_letter(
        &self,
        letter: &Letter,
        x: f32,
        y: f32,
        word_index: usize,
        char_index: usize,
    ) -> TextDimensions {
        let style = &self.style;

        let dimensions = text::measure_text(
            &letter.letter.to_string(),
            Some(&self.font),
            *style.font_size.borrow() as u16,
            1.0,
        );

        let p_x = match &style.padding_x {
            Some(p) => p.get(style),
            _ => 0.0,
        };

        let o_x = match &style.offset_x {
            Some(p) => p.get(style),
            _ => 0.0,
        };

        let p_y = match &style.padding_y {
            Some(p) => p.get(style),
            _ => 0.0,
        };

        let o_y = match &style.offset_y {
            Some(p) => p.get(style),
            _ => 0.0,
        };

        let x = x + p_x + o_x;
        let y = y + p_y + o_y;

        text::draw_text_ex(
            &letter.letter.to_string(),
            x,
            y,
            TextParams {
                font: Some(&self.font),
                font_size: *style.font_size.borrow() as u16,
                color: *letter.color.borrow(),
                ..TextParams::default()
            },
        );

        if (self.state.word_index > letter.word_id
            || self.state.word_index == letter.word_id && self.state.char_index > letter.char_id)
            && letter.letter != ' '
            && *letter.color.borrow() != *style.theme.text.borrow()
        {
            shapes::draw_line(
                x,
                y + 0.2 * *style.font_size.borrow(),
                x + dimensions.width,
                y + 0.2 * *style.font_size.borrow(),
                0.05 * *style.font_size.borrow(),
                *style.theme.error.borrow(),
            );
        }

        if letter.char_id == char_index && letter.word_id == word_index {
            text::draw_text_ex(
                "|",
                x - dimensions.width / 2.0,
                y,
                TextParams {
                    font: Some(&self.font),
                    font_size: *style.font_size.borrow() as u16,
                    color: *style.theme.text.borrow(),
                    ..TextParams::default()
                },
            );
        }

        dimensions
    }
}
