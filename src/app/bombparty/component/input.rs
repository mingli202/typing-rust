use itertools::Itertools;
use std::fmt::Display;
use tokio::time::Instant;

use macroquad::input::KeyCode;
use macroquad::{input, shapes, text};

use crate::app::bombparty::style::Style;

use super::container::{Container, Padding};
use super::text::Text;
use super::Component;

pub struct Input {
    pub style: Style,
    pub value: Vec<Line>,
    pub focused: bool,
    pub location: Location,
    pub hold_time: Instant,
    pub last_key_pressed: Option<KeyCode>,
    pub last_char_pressed: Option<char>,
    container: Container,
    cursor_timer: Instant,
}

impl Input {
    pub fn new(style: Style) -> Self {
        let fsize = *style.font_size.borrow();

        Input {
            value: vec![Line::default()],
            focused: true,
            location: Location::new(0, 0, 0),
            hold_time: Instant::now(),
            cursor_timer: Instant::now(),
            last_key_pressed: None,
            last_char_pressed: None,
            container: Container {
                style: style.clone(),
                child: Box::new(Text::new(style.clone(), "".to_string())),
                padding: Padding::new(fsize / 3.0),
            },
            style,
        }
    }

    fn add_last(&mut self, c: char) {
        match c {
            '\u{000d}' => {
                self.value.push(Line::default());
            }
            ' ' => {
                if self.value.last().is_none() {
                    self.value.push(Line::default());
                }
                self.value.last_mut().unwrap().words.push(Word::default());
            }
            _ => {
                if self.value.last().is_none() {
                    self.value.push(Line::default());
                }
                if self.value.last().unwrap().words.last().is_none() {
                    self.value.last_mut().unwrap().words.push(Word::default());
                }
                self.value
                    .last_mut()
                    .unwrap()
                    .words
                    .last_mut()
                    .unwrap()
                    .letters
                    .push(Letter::new(c));
            }
        };
    }

    fn remove_last(&mut self) {
        if let Some(line) = self.value.last_mut() {
            if let Some(word) = line.words.last_mut() {
                if word.letters.pop().is_none() {
                    line.words.pop();

                    if line.words.is_empty() {
                        self.value.pop();
                    }
                }
            }
        }
    }

    fn remove_word(&mut self) {
        if let Some(line) = self.value.last_mut() {
            line.words.pop();

            if line.words.is_empty() {
                self.value.pop();
            }
        }
    }

    fn remove_line(&mut self) {
        self.value.pop();
    }

    fn draw_cursor(&self) {
        let Style {
            mut x,
            mut y,
            font_size,
            theme,
            font,
            ..
        } = &self.style;

        let font_size = *font_size.borrow();
        x += font_size / 3.0;
        y += font_size / 3.0;

        if let Some(line) = self.value.last() {
            let width = text::measure_text(
                &line.to_string().replace("\t", "    "),
                font.as_deref(),
                font_size as u16,
                1.0,
            )
            .width;

            x += width;
            y += (self.value.len() - 1) as f32 * font_size;
        }
        shapes::draw_line(x, y, x, y + font_size, 2.0, *theme.text.borrow());
    }
}

impl Component for Input {
    fn build(&mut self) {
        self.container.build();
        self.style.width = self.container.style.width;
        self.style.height = self.container.style.height;
    }
    fn on_click_in(&mut self) {
        self.focused = true;
    }
    fn on_click_out(&mut self) {
        self.focused = false;
    }

    fn refresh(&mut self) {
        if self.focused {
            let keys = input::get_keys_down();

            let key = match input::get_last_key_pressed() {
                Some(k) => {
                    self.last_key_pressed = None;
                    self.last_char_pressed = None;
                    Some(k)
                }
                None => {
                    if let Some(k) = self.last_key_pressed {
                        if keys.contains(&k) {
                            if self.hold_time.elapsed().as_millis() > 500 {
                                Some(k)
                            } else {
                                None
                            }
                        } else {
                            self.last_key_pressed = None;
                            self.last_char_pressed = None;
                            None
                        }
                    } else {
                        None
                    }
                }
            };

            if let Some(key) = key {
                if self.last_key_pressed.is_none() {
                    self.last_key_pressed = Some(key);
                    self.hold_time = Instant::now();
                }

                match key {
                    KeyCode::Backspace => {
                        if keys.contains(&KeyCode::LeftAlt) || keys.contains(&KeyCode::RightAlt) {
                            self.remove_word();
                        } else if keys.contains(&KeyCode::LeftSuper)
                            || keys.contains(&KeyCode::RightSuper)
                        {
                            self.remove_line();
                        } else {
                            self.remove_last();
                        }
                    }
                    _ => {
                        let c = match input::get_char_pressed() {
                            Some(_c) => Some(_c),
                            None => self.last_char_pressed,
                        };

                        if let Some(c) = c {
                            self.last_char_pressed = Some(c);

                            self.add_last(c);
                        }
                    }
                }
                input::clear_input_queue();
                self.container.child = Box::new(Text::new(self.style.clone(), self.to_string()));
                self.build();
                self.draw_cursor();
                self.cursor_timer = Instant::now();
            } else {
                let t = self.cursor_timer.elapsed().as_millis();
                if t < 500 {
                    self.draw_cursor();
                }
                if t > 1000 {
                    self.cursor_timer = Instant::now();
                }
            }
        }

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

        self.container.style.x = self.style.x;
        self.container.style.y = self.style.y;
        self.container.refresh();
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

impl Default for Line {
    fn default() -> Self {
        Line {
            words: vec![Word::default()],
        }
    }
}

impl Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.words.iter().map(|w| w.to_string()).join(" "))
    }
}

#[derive(Clone, Debug, Default)]
pub struct Word {
    pub letters: Vec<Letter>,
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
/// Represents the current location of the cursor
pub struct Location {
    /// the current line
    pub line_index: usize,
    /// the current word in the line
    pub word_index: usize,
    /// the current letter in the word
    pub letter_index: usize,
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
        assert_eq!(input.value.len(), 1, "empty vector");
        // assert_eq!(input.location.line_index, 0);
        // assert_eq!(input.location.word_index, 0);
        // assert_eq!(input.location.letter_index, 0);
    }

    #[test]
    fn input_add_characters() {
        let mut input = input();
        input.add_last('a');
        input.add_last('b');
        input.add_last('c');
        assert_eq!(input.to_string(), "abc");
        // assert_eq!(input.location.line_index, 0);
        // assert_eq!(input.location.word_index, 0);
        // assert_eq!(input.location.letter_index, 2);
    }

    #[test]
    fn input_add_characters_2() {
        let mut input = input();
        input.add_last('a');
        input.add_last('b');
        input.add_last(' ');
        assert_eq!(input.to_string(), "ab ");
        // assert_eq!(input.location.line_index, 0);
        // assert_eq!(input.location.word_index, 1);
        // assert_eq!(input.location.letter_index, 0);

        input.add_last('c');
        input.add_last('d');
        input.add_last('e');
        assert_eq!(input.to_string(), "ab cde");
        // assert_eq!(input.location.line_index, 0);
        // assert_eq!(input.location.word_index, 1);
        // assert_eq!(input.location.letter_index, 2);
    }

    #[test]
    fn input_add_line() {
        let mut input = input();

        input.add_last('a');
        input.add_last('b');
        input.add_last('c');
        assert_eq!(input.to_string(), "abc");

        input.add_last('\u{000d}');
        assert_eq!(input.to_string(), "abc\n");

        input.add_last('a');
        input.add_last('b');
        input.add_last('c');
        assert_eq!(input.to_string(), "abc\nabc");
        // assert_eq!(Location::new(1, 0, 2), input.location);

        input.add_last(' ');
        input.add_last('d');
        input.add_last('e');
        assert_eq!(input.to_string(), "abc\nabc de");
        // assert_eq!(Location::new(1, 1, 1), input.location);
    }

    #[test]
    fn input_add_empty_word() {
        let mut input = input();

        input.add_last(' ');
        input.add_last('a');
        input.add_last('b');

        assert_eq!(input.to_string(), " ab");
        // assert_eq!(Location::new(0, 1, 1), input.location);

        input.add_last(' ');
        input.add_last(' ');
        input.add_last(' ');
        assert_eq!(input.to_string(), " ab   ");
        // assert_eq!(Location::new(0, 4, 0), input.location);

        input.add_last('c');
        assert_eq!(input.to_string(), " ab   c");
        // assert_eq!(Location::new(0, 4, 0), input.location);
    }

    #[test]
    fn input_add_empty_line() {
        let mut input = input();

        input.add_last('\u{000d}');
        input.add_last('a');
        input.add_last('b');
        assert_eq!(input.to_string(), "\nab");
        // assert_eq!(Location::new(1, 0, 1), input.location);

        input.add_last('\u{000d}');
        input.add_last('\u{000d}');
        input.add_last('\u{000d}');
        assert_eq!(input.to_string(), "\nab\n\n\n");
        // assert_eq!(Location::new(4, 0, 0), input.location);

        input.add_last('a');
        input.add_last('b');
        input.add_last(' ');
        input.add_last('a');
        assert_eq!(input.to_string(), "\nab\n\n\nab a");
        // assert_eq!(Location::new(4, 1, 0), input.location);
    }

    #[test]
    fn input_delete_char() {
        let mut input = input();
        input.add_last('a');
        input.add_last('b');
        input.add_last('c');
        input.add_last('d');
        assert_eq!(input.to_string(), "abcd");

        input.remove_last();
        assert_eq!(input.to_string(), "abc");
        // assert_eq!(Location::new(0, 0, 2), input.location);

        input.remove_last();
        input.remove_last();
        input.remove_last();
        assert_eq!(input.to_string(), "");
        // assert_eq!(Location::new(0, 0, 0), input.location);

        input.remove_last();
        assert_eq!(input.to_string(), "");
    }

    #[test]
    fn input_delete_line() {
        let mut input = input();
        input.add_last('a');
        input.add_last('b');
        input.add_last('c');

        input.add_last('\u{000d}');
        assert_eq!(input.to_string(), "abc\n");

        input.add_last('a');
        assert_eq!(input.to_string(), "abc\na");

        input.remove_last();
        assert_eq!(input.to_string(), "abc\n");

        input.remove_last();
        assert_eq!(input.to_string(), "abc");

        input.remove_last();
        input.remove_last();
        input.remove_last();
        input.remove_last();
        assert_eq!(input.to_string(), "");
    }

    #[test]
    fn input_remove_word() {
        let mut input = input();
        input.add_last('a');
        input.add_last('b');
        input.add_last('c');
        input.add_last(' ');
        input.add_last('b');
        input.add_last('c');
        input.add_last('\u{000d}');
        input.add_last('a');
        input.add_last('b');
        input.add_last('c');
        input.add_last(' ');
        input.add_last('b');
        input.add_last('c');
        assert_eq!(input.to_string(), "abc bc\nabc bc");

        input.remove_word();
        assert_eq!(input.to_string(), "abc bc\nabc");

        input.remove_word();
        assert_eq!(input.to_string(), "abc bc");

        input.remove_word();
        assert_eq!(input.to_string(), "abc");

        input.remove_word();
        assert_eq!(input.to_string(), "");

        input.remove_word();
    }

    #[test]
    fn input_remove_line() {
        let mut input = input();
        input.add_last('a');
        input.add_last('b');
        input.add_last('\u{000d}');
        input.add_last('a');
        input.add_last('b');
        input.add_last('\u{000d}');
        input.add_last('a');
        input.add_last('b');
        assert_eq!(input.to_string(), "ab\nab\nab");

        input.remove_line();
        assert_eq!(input.to_string(), "ab\nab");

        input.remove_line();
        assert_eq!(input.to_string(), "ab");

        input.remove_line();
        assert_eq!(input.to_string(), "");
        input.remove_line();
    }
}
