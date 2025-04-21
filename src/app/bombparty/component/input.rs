use macroquad::input::KeyCode;
use macroquad::text::{TextDimensions, TextParams};
use macroquad::{input, shapes, text};

use crate::app::bombparty::style::Style;

use super::text::Text;
use super::Component;

pub struct Input {
    pub style: Style,
    pub value: String,
    pub focused: bool,
}

impl Input {
    pub fn new(style: Style) -> Self {
        let initial_value = "asdf".to_string();

        let TextDimensions {
            width,
            height,
            offset_y,
        } = text::measure_text(
            &initial_value[..],
            style.font.as_deref(),
            *style.font_size.borrow() as u16,
            1.0,
        );

        Input {
            style: Style {
                width,
                height: height + offset_y,
                ..style
            },
            value: initial_value,
            focused: false,
        }
    }
    fn add_letter(&mut self, c: char) {
        let Location {
            line_index,
            word_index,
            ..
        } = self.location;

        let word = &mut self.value[line_index].line[word_index].word;
        word.push(Letter::new(c));

        self.location.letter_index += 1;
    }
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
            if let Some(key) = input::get_last_key_pressed() {
                match key {
                    KeyCode::Backspace => {
                        self.value.pop();
                    }
                    KeyCode::Tab => self.value += "\t",

                    KeyCode::Enter => self.value += "\n",
                    _ => {
                        if let Some(c) = input::get_char_pressed() {
                            match c {
                                '\u{0008}' => {
                                    self.value.pop();
                                }
                                _ => self.value += &c.to_string(),
                            };
                        }
                    }
                }
                input::clear_input_queue();
            }
        }

        let mut text = Text::new(self.style.clone(), self.value.clone());
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
    pub line: Vec<Word>,
}

impl Line {
    pub fn new() -> Self {
        Line { line: vec![] }
    }
}

impl Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.line.iter().map(|w| w.to_string()).join(" "))
    }
}

#[derive(Clone, Debug)]
pub struct Word {
    pub word: Vec<Letter>,
}

impl Word {
    pub fn new() -> Self {
        Word { word: vec![] }
    }
}

impl Display for Word {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.word.iter().map(|l| l.to_string()).join(""))
    }
}

#[derive(Clone, Debug)]
pub struct Letter {
    pub letter: char,
}

impl Letter {
    pub fn new(c: char) -> Self {
        Letter { letter: c }
    }
}

impl Display for Letter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.letter)
    }
}

#[derive(Clone, Debug)]
pub struct Location {
    pub line_index: usize,
    pub word_index: usize,
    pub letter_index: usize,
}
