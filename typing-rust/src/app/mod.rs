use crate::data_provider::{Data, Quote};
use crate::Config;
use macroquad::color::Color;
use macroquad::text::{load_ttf_font, Font};
use macroquad::window;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::error::Error;
use std::fmt::Display;
use std::rc::Rc;
use std::time::Duration;
mod theme;

mod style;
pub use style::{BorderParams, Style, Value};

pub use self::theme::*;

mod text;
pub mod util;

pub mod bombparty;
mod endscreen;
mod focus;
mod theme_select;
mod typing_test;

pub struct App {
    style: Style,
    data: Data,
    config: Config,
    state: AppState,
    pub typing_font: Rc<Font>,
    pub font: Rc<Font>,
}

impl App {
    pub async fn new(data: Data, config: Config) -> Self {
        let typing_font: Font =
            load_ttf_font("/Users/vincentliu/Library/Fonts/SauceCodeProNerdFontMono-Regular.ttf")
                .await
                .unwrap();

        let font: Rc<Font> = Rc::new(
            load_ttf_font("/System/Library/Fonts/Helvetica.ttc")
                .await
                .unwrap(),
        );

        App {
            data,
            style: Style {
                font_size: Rc::new(RefCell::new(config.font_size)),
                width: Value::Relative(Box::new(|_| window::screen_width())),
                height: Value::Relative(Box::new(|_| window::screen_height())),
                theme: Theme::get_theme(&config.theme),
                font: Some(Rc::clone(&font)),
                ..Style::default()
            },
            state: AppState {
                mode: config.mode.clone(),
                ..AppState::default()
            },
            config,
            typing_font: Rc::new(typing_font),
            font,
        }
    }

    pub async fn main_loop(&mut self) -> Result<(), Box<dyn Error>> {
        self.state.mode.next(&mut self.data);

        loop {
            match self.state.screen {
                Screen::TypingTest => typing_test::run(self).await,
                Screen::End => endscreen::run(self).await,
                Screen::ThemeSelect => theme_select::run(self).await,
                Screen::Bombparty => bombparty::run(self).await,
            };
        }
    }
}

pub struct AppState {
    wpm: f32,
    mode: Mode,
    screen: Screen,
    incremental_wpm: Vec<(Duration, f32)>,
    max_wpm: f32,
    time: Duration,
    accuracy: i32,
}

impl AppState {
    pub fn add_wpm(&mut self, time: Duration, wpm: f32) {
        if wpm > self.max_wpm {
            self.max_wpm = wpm;
        }
        self.incremental_wpm.push((time, wpm));
    }
}

impl Default for AppState {
    fn default() -> Self {
        AppState {
            wpm: 0.0,
            mode: Mode::Words {
                n: 0,
                s: "".to_string(),
            },
            screen: Screen::Bombparty,
            incremental_wpm: vec![],
            max_wpm: 0.0,
            time: Duration::from_secs(0),
            accuracy: 0,
        }
    }
}

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
enum Screen {
    TypingTest,
    End,
    ThemeSelect,
    Bombparty,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Mode {
    Words {
        n: usize,

        #[serde(skip)]
        s: String,
    },
    Quote(Quote),
}

impl Mode {
    pub fn new(data: &mut Data) -> Self {
        // Self::with_quote(data)
        Self::with_words(data, 10)
    }

    pub fn with_quote(data: &mut Data) -> Self {
        Mode::Quote(data.get_random_quote().clone())
    }

    pub fn with_words(data: &mut Data, n: usize) -> Self {
        let words = data.get_n_random_words(n);
        Mode::Words {
            n,
            s: words
                .iter()
                .map(|w| w.to_string())
                .collect::<Vec<String>>()
                .join(" "),
        }
    }

    pub fn get_inner(&self) -> String {
        match self {
            Mode::Words { s, .. } => s.to_string(),
            Mode::Quote(q) => q.quote.clone(),
        }
    }

    pub fn next(&mut self, data: &mut Data) {
        let new_mode = match self {
            Mode::Words { n, .. } => Mode::Words {
                s: data
                    .get_n_random_words(*n)
                    .iter()
                    .map(|s| &(*s)[..])
                    .collect::<Vec<&str>>()
                    .join(" "),
                n: *n,
            },
            Mode::Quote(_) => Mode::Quote(data.get_random_quote().clone()),
        };
        *self = new_mode;
    }

    pub fn get_name(&self) -> String {
        match self {
            Mode::Quote(_) => "Quote".to_string(),
            Mode::Words { .. } => "Words".to_string(),
        }
    }
}

impl Default for Mode {
    fn default() -> Self {
        Mode::Quote(Quote {
            source: "".to_string(),
            quote: "".to_string(),
        })
    }
}

impl Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Mode::Words { n, .. } => write!(f, "{} {}", n, if *n != 1 { "Words" } else { "Word" }),
            Mode::Quote(Quote { source, .. }) => write!(f, "{}", source),
        }
    }
}

impl PartialEq for Mode {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Mode::Words { n, .. } => match other {
                Mode::Words { n: m, .. } => n == m,
                _ => false,
            },
            Mode::Quote(_) => matches!(other, Mode::Quote(_)),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Word {
    pub letters: Vec<Letter>,
    pub is_error: bool,
    pub id: usize,
    pub word: String,
    pub last_typed: usize,
}

impl Word {
    pub fn from_str(style: &Style, text: &str, id: usize) -> Word {
        Word {
            letters: text
                .chars()
                .enumerate()
                .map(|(i, letter)| Letter {
                    letter,
                    color: Rc::clone(&style.theme.ghost),
                    char_id: i,
                    word_id: id,
                })
                .collect(),
            is_error: false,
            id,
            word: text.to_string(),
            last_typed: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Letter {
    pub letter: char,
    pub color: Rc<RefCell<Color>>,
    pub char_id: usize,
    pub word_id: usize,
}
