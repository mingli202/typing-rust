use std::cmp::Ordering;
use std::rc::Rc;
use std::time::Instant;

use macroquad::text::TextDimensions;
use macroquad::{shapes, text, window};

use crate::app::Word;
use crate::app::{theme::Theme, BorderParams, Letter, Style, Value};

pub struct TextBoxState {
    pub letters: Vec<Letter>,
    pub words: Vec<Word>,
    pub index: usize,
    pub word_index: usize,
    pub char_index: usize,
    pub time_started: Instant,
    pub started: bool,
}

pub struct TextBox {
    pub style: Style,
    pub state: TextBoxState,
}

impl TextBox {
    pub fn new(style: &Style, text: String) -> TextBox {
        let letters: Vec<Letter> = text
            .chars()
            .enumerate()
            .map(|(id, c)| Letter {
                letter: c,
                color: Rc::clone(&style.theme.ghost),
                char_id: id,
                word_id: id,
            })
            .collect();

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
                x: Value::Relative(Box::new(|_| (0.5 * window::screen_width()) / 2.0)),
                y: Value::Relative(Box::new(move |_| {
                    (window::screen_height() - *f1.borrow() * 3.0) / 2.0
                })),
                width: Value::Relative(Box::new(|_| window::screen_width() / 2.0)),
                height: Value::Relative(Box::new(move |_| *f2.borrow() * 3.0)),
                clip: true,
                ..Style::default()
            },
            state: TextBoxState {
                letters,
                index: 0,
                word_index: 0,
                char_index: 0,
                time_started: Instant::now(),
                started: false,
                words,
            },
        }
    }

    pub fn refresh(&mut self, text: String) {
        let letters: Vec<Letter> = text
            .chars()
            .enumerate()
            .map(|(id, c)| Letter {
                letter: c,
                color: Rc::clone(&self.style.theme.ghost),
                char_id: id,
                word_id: id,
            })
            .collect();

        self.state.letters = letters;
        self.state.index = 0;
        self.state.started = false;

        self.style.offset_y = None;
    }

    pub fn on_type(&mut self, c: char) -> bool {
        if self.state.word_index == self.state.words.len() - 1
            && self.state.char_index == self.state.words.last().unwrap().letters.len() - 1
        {
            return true;
        }

        if c == ' ' {
            // move to the next word
            self.state.word_index += 1;
            self.state.char_index = 0;
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
        false
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
            return;
        }

        self.state.char_index -= 1;
        self.state.words[self.state.word_index].last_typed = self.state.char_index;

        // check if we are deleting overflow
        if self.state.char_index >= self.state.words[self.state.word_index].word.len() {
            self.state.words[self.state.word_index].letters.pop();
        } else {
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
            -(left as f32 * *self.style.font_size.borrow()),
        ));
    }

    pub fn get_wpm(&self) -> u16 {
        let time_passed: u128 = self.state.time_started.elapsed().as_millis();
        let mut wrongs = 0.0;
        let mut is_word_wrong = false;

        for letter in &self.state.letters {
            if *letter.color.borrow() == *self.style.theme.error.borrow() && !is_word_wrong {
                wrongs += 1.0;
                is_word_wrong = true;
            }
            if letter.letter == ' ' {
                is_word_wrong = false;
            }
        }

        ((1000.0 * 60.0 * (self.state.letters.len() as f32 / 5.0 - wrongs)) as u128 / time_passed)
            as u16
    }

    pub fn update(&mut self) {
        self.style.draw_bg();

        //let line_breaks =
        //    app::text::print_letters_wrap(&self.style, &self.state.letters, self.state.index);
        let line_breaks = self.print_words();
        self.update_position(&line_breaks);

        self.style.draw_mask();

        if self.state.index > 0 && !self.state.started {
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

            let TextDimensions { width, .. } =
                text::measure_text(&l, None, *self.style.font_size.borrow() as u16, 1.0);

            if width > self.style.width.get(&self.style) - 2.0 * p_x {
                self.print_letters(
                    &line,
                    self.style.x.get(&self.style),
                    self.style.y.get(&self.style) + lines as f32 * *self.style.font_size.borrow(),
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

        self.print_letters(
            &line,
            self.style.x.get(&self.style),
            self.style.y.get(&self.style) + lines as f32 * *self.style.font_size.borrow(),
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
            None,
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
            None,
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

        text::draw_text(
            &letter.letter.to_string(),
            x,
            y,
            *style.font_size.borrow(),
            *letter.color.borrow(),
        );

        if *letter.color.borrow() == *style.theme.error.borrow() {
            shapes::draw_line(
                x,
                y + 0.2 * *style.font_size.borrow(),
                x + dimensions.width,
                y + 0.2 * *style.font_size.borrow(),
                0.05 * *style.font_size.borrow(),
                *letter.color.borrow(),
            );
        }

        if letter.char_id == char_index && letter.word_id == word_index {
            text::draw_text(
                "|",
                x - dimensions.width / 2.0,
                y,
                *style.font_size.borrow(),
                *style.theme.text.borrow(),
            );
        }

        dimensions
    }
}
