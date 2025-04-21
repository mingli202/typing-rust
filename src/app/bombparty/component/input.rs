use itertools::Itertools;
use std::fmt::Display;
use std::rc::Rc;

use macroquad::input::KeyCode;
use macroquad::{input, shapes};

use crate::app::bombparty::style::Style;

use super::text::Text;
use super::Component;

pub struct Input {
    pub style: Style,
    pub value: Vec<Line>,
    pub focused: bool,
    pub location: Location,
}

impl Input {
    pub fn new(style: Style) -> Self {
        Input {
            style,
            value: vec![],
            focused: true,
            location: Location {
                line_index: 0,
                word_index: 0,
                letter_index: 0,
            },
        }
    }

    fn add_letter(&mut self, c: char) {
        match c {
            '\u{000d}' => {
                self.value.push(Line::new());
                self.location.line_index += 1;
                self.location.word_index = 0;
                self.location.letter_index = 0;
            }
            ' ' => {
                let line = &mut self.value[self.location.line_index].words;
                line.push(Word::new());
                self.location.word_index += 1;
                self.location.letter_index = 0;
            }
            _ => {
                let Location {
                    line_index,
                    word_index,
                    ..
                } = self.location;

                if line_index >= self.value.len() {
                    self.value.push(Line::new());
                }

                if word_index >= self.value[line_index].words.len() {
                    self.value[line_index].words.push(Word::new());
                }

                let word = &mut self.value[line_index].words[word_index].letters;
                word.push(Letter::new(c));

                self.location.letter_index += 1;
            }
        };
    }

    fn remove_letter(&mut self) {
        let Location {
            line_index,
            word_index,
            letter_index,
        } = &mut self.location;

        if *line_index >= self.value.len() {
            return;
        }

        if self.value[*line_index].words[*word_index]
            .letters
            .pop()
            .is_none()
        {
            if self.value[*line_index].words.pop().is_none() {
                if self.value.pop().is_some() {
                    *line_index -= 1;

                    *word_index = self.value[*line_index].words.len();
                    if *word_index > 0 {
                        *word_index -= 1;
                    }
                }
            } else {
                *word_index -= 1;
            }

            *letter_index = self.value[*line_index].words[*word_index].letters.len();
            if *letter_index > 0 {
                *letter_index -= 1;
            }
        } else {
            *letter_index -= 1;
        }
    }

    fn remove_word(&mut self) {}
}

impl Component for Input {
    fn on_click_in(&mut self) {
        self.focused = true;
    }
    fn on_click_out(&mut self) {
        self.focused = false;
    }

    fn refresh(&mut self) {
        if self.focused {
            let keys = input::get_keys_down();

            if let Some(key) = input::get_last_key_pressed() {
                match key {
                    KeyCode::Backspace => {
                        if keys.contains(&KeyCode::LeftAlt) || keys.contains(&KeyCode::RightAlt) {
                            self.remove_word();
                        } else {
                            self.remove_letter();
                        }
                    }
                    _ => {
                        if let Some(c) = input::get_char_pressed() {
                            self.add_letter(c);
                        }
                    }
                }
                input::clear_input_queue();
            }
        }

        let mut text = Text::new(self.style.clone(), self.to_string());
        text.refresh();

        self.style.width = text.style.width;
        self.style.height = text.style.height;

        shapes::draw_rectangle_lines(
            self.style.x,
            self.style.y,
            self.style.width,
            self.style.height,
            2.0,
            if self.focused {
                *self.style.theme.text.borrow()
            } else {
                *self.style.theme.ghost.borrow()
            },
        );
    }

    fn get_style(&self) -> &Style {
        &self.style
    }
    fn get_style_mut(&mut self) -> &mut Style {
        &mut self.style
    }
}

impl Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value.iter().map(|l| l.to_string()).join("\n"))
    }
}

#[derive(Clone, Debug)]
pub struct Line {
    pub words: Vec<Word>,
}

impl Line {
    pub fn new() -> Self {
        Line { words: vec![] }
    }
}

impl Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.words.iter().map(|w| w.to_string()).join(" "))
    }
}

#[derive(Clone, Debug)]
pub struct Word {
    pub letters: Vec<Letter>,
}

impl Word {
    pub fn new() -> Self {
        Word { letters: vec![] }
    }
}

impl Display for Word {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.letters.iter().map(|l| l.to_string()).join(""))
    }
}

#[derive(Clone, Debug)]
pub struct Letter {
    pub c: char,
}

impl Letter {
    pub fn new(c: char) -> Self {
        Letter { c }
    }
}

impl Display for Letter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.c)
    }
}

#[derive(Clone, Debug)]
pub struct Location {
    pub line_index: usize,
    pub word_index: usize,
    pub letter_index: usize,
}

#[cfg(test)]
mod tests {
    use super::{Input, Style};

    fn input() -> Input {
        Input::new(Style::default())
    }

    #[test]
    fn input_constructor() {
        let input = input();
        assert!(input.value.is_empty(), "empty vector");
        assert_eq!(input.location.line_index, 0);
        assert_eq!(input.location.word_index, 0);
        assert_eq!(input.location.letter_index, 0);
    }

    #[test]
    fn input_add_characters() {
        let mut input = input();
        input.add_letter('a');
        assert_eq!(input.to_string(), "a");
    }
}
