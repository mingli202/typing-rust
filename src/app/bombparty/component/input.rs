use itertools::Itertools;
use std::fmt::Display;

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
            location: Location::new(0, 0, 0),
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
                if self.location.line_index >= self.value.len() {
                    self.value.push(Line::new());
                }
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
    pub line_index: usize,   // current line
    pub word_index: usize,   // current work in line
    pub letter_index: usize, // current letter in word + 1
}

impl Location {
    pub fn new(line_index: usize, word_index: usize, letter_index: usize) -> Self {
        Location {
            line_index,
            word_index,
            letter_index,
        }
    }
}

impl PartialEq for Location {
    fn eq(&self, other: &Self) -> bool {
        self.line_index == other.line_index
            && self.word_index == other.word_index
            && self.letter_index == other.letter_index
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        input.add_letter('b');
        input.add_letter('c');
        assert_eq!(input.to_string(), "abc");
        assert_eq!(input.location.line_index, 0);
        assert_eq!(input.location.word_index, 0);
        assert_eq!(input.location.letter_index, 3);
    }

    #[test]
    fn input_add_characters_2() {
        let mut input = input();
        input.add_letter('a');
        input.add_letter('b');
        input.add_letter(' ');
        assert_eq!(input.to_string(), "ab ");
        assert_eq!(input.location.line_index, 0);
        assert_eq!(input.location.word_index, 1);
        assert_eq!(input.location.letter_index, 0);

        input.add_letter('c');
        input.add_letter('d');
        input.add_letter('e');
        assert_eq!(input.to_string(), "ab cde");
        assert_eq!(input.location.line_index, 0);
        assert_eq!(input.location.word_index, 1);
        assert_eq!(input.location.letter_index, 3);
    }

    #[test]
    fn input_add_line() {
        let mut input = input();

        input.add_letter('a');
        input.add_letter('b');
        input.add_letter('c');
        assert_eq!(input.to_string(), "abc");

        input.add_letter('\u{000d}');
        assert_eq!(input.to_string(), "abc\n");

        input.add_letter('a');
        input.add_letter('b');
        input.add_letter('c');
        assert_eq!(input.to_string(), "abc\nabc");
        assert_eq!(Location::new(1, 0, 3), input.location);

        input.add_letter(' ');
        input.add_letter('d');
        input.add_letter('e');
        assert_eq!(input.to_string(), "abc\nabc de");
        assert_eq!(Location::new(1, 1, 2), input.location);
    }

    #[test]
    fn input_add_empty_word() {
        let mut input = input();

        input.add_letter(' ');
        input.add_letter('a');
        input.add_letter('b');

        assert_eq!(input.to_string(), " ab");
        assert_eq!(Location::new(0, 1, 2), input.location);

        input.add_letter(' ');
        input.add_letter(' ');
        input.add_letter(' ');
        assert_eq!(input.to_string(), " ab   ");
        assert_eq!(Location::new(0, 4, 0), input.location);

        input.add_letter('c');
        assert_eq!(input.to_string(), " ab   c");
        assert_eq!(Location::new(0, 4, 1), input.location);
    }

    #[test]
    fn input_add_empty_line() {
        let mut input = input();

        input.add_letter('\u{000d}');
        input.add_letter('a');
        input.add_letter('b');
        assert_eq!(input.to_string(), "\nab");
        assert_eq!(Location::new(1, 0, 2), input.location);

        input.add_letter('\u{000d}');
        input.add_letter('\u{000d}');
        input.add_letter('\u{000d}');
        assert_eq!(input.to_string(), "\nab\n\n\n");
        assert_eq!(Location::new(4, 0, 0), input.location);

        input.add_letter('a');
        input.add_letter('b');
        input.add_letter(' ');
        input.add_letter('a');
        assert_eq!(input.to_string(), "\nab\n\n\nab a");
        assert_eq!(Location::new(4, 1, 1), input.location);
    }
}
