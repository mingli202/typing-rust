use std::cell::RefCell;
use std::rc::Rc;

use macroquad::input;

use crate::data_provider::{Data, Quote};
use crate::Config;

use super::focus::TypingTestFocus::{self, *};

use super::style::Style;
use super::theme::Theme;
use super::typing_test::textbox::TextBox;

pub struct State {
    pub mode: Rc<RefCell<Mode>>,
    pub wpm: Rc<RefCell<u16>>,
    pub screen: Rc<RefCell<Screen>>,
    pub style: Style,
    pub data: Data,
    pub config: Config,

    pub typingtest: TypingtestState,
}

impl State {
    pub fn new(data: Data, config: Config) -> Self {
        State {
            mode: Rc::new(RefCell::new(Mode::from_quote(
                data.get_random_quote().clone(),
            ))),
            wpm: Rc::new(RefCell::new(0)),
            style: Style {
                font_size: Rc::new(RefCell::new(config.font_size)),
                theme: Theme::get_theme(&config.theme),
                ..Style::default()
            },
            config,
            data,
            screen: Rc::new(RefCell::new(Screen::TypingTest)),
            typingtest: TypingtestState::default(),
        }
    }

    pub fn dispatch(&self, action: Action) {
        match action {
            Action::FontChange(n) => *self.style.font_size.borrow_mut() += n,
            Action::ScreenChange(scr) => *self.screen.borrow_mut() = scr,
            Action::WpmChange(f) => *self.wpm.borrow_mut() = f,

            // typing test
            Action::TypingtestClick(typingbox) => {
                input::clear_input_queue();
                match *self.typingtest.focus.borrow() {
                    NextButton => {
                        self.mode.borrow_mut().next(&self.data);
                        typingbox.refresh(self.mode.borrow().get_text());
                        *self.typingtest.focus.borrow_mut() = Nothing;
                    }
                    RestartButton => {
                        typingbox.refresh(self.mode.borrow().get_text());
                        *self.typingtest.focus.borrow_mut() = Nothing;
                    }
                    ThemeButton => {
                        *self.screen.borrow_mut() = Screen::ThemeSelect;
                    }
                    _ => (),
                }
            }
            Action::TypingTestFocusChange(focus) => *self.typingtest.focus.borrow_mut() = focus,
        }
    }
}

pub enum Action<'a> {
    FontChange(f32),
    ScreenChange(Screen),
    WpmChange(u16),

    TypingtestClick(&'a mut TextBox),
    TypingTestFocusChange(TypingTestFocus),
}

#[derive(Default)]
pub struct TypingtestState {
    pub focus: Rc<RefCell<TypingTestFocus>>,
}

pub enum TypingtestAction {
    Click,
}

#[derive(Default)]
pub enum Screen {
    #[default]
    TypingTest,
    EndScreen,
    ThemeSelect,
}

pub enum Mode {
    Quote(Quote),
    Words(String),
}

impl Mode {
    pub fn from_quote(quote: Quote) -> Self {
        Mode::Quote(quote)
    }

    pub fn from_words(words: Vec<&String>) -> Self {
        Mode::Words(words.iter().fold(String::new(), |acc, s| acc + s))
    }

    pub fn get_text(&self) -> &str {
        match self {
            Mode::Words(words) => words,
            Mode::Quote(quote) => &quote.quote,
        }
    }

    pub fn next(&mut self, data: &Data) {
        *self = match self {
            Mode::Words(words) => Mode::from_words(data.get_n_random_words(words.len())),
            Mode::Quote(_) => Mode::from_quote(data.get_random_quote().clone()),
        }
    }
}
