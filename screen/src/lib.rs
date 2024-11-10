use macroquad::{input, window};
use std::error::Error;
use std::rc::Rc;

use data_provider::Data;

mod theme;
use theme::Theme;

mod component;
use component::TextBox;
use component::{Component, Style, Value};

pub enum Mode {
    WordCount(usize),
    TimeSec(usize),
    Quote,
}

impl Default for Mode {
    fn default() -> Self {
        Mode::TimeSec(30)
    }
}

enum State {
    Typing(Mode),
    EndScreen,
}

pub struct Screen {
    font_size: f32,
    theme: Theme,
    mode: State,
    data: Data,
    components: Vec<Box<dyn Component>>,
}

impl Screen {
    pub fn new(
        data: Data,
        theme: Option<Theme>,
        mode: Option<Mode>,
        font_size: Option<f32>,
    ) -> Self {
        let mut initial = Screen {
            data,
            theme: theme.unwrap_or_default(),
            mode: State::Typing(mode.unwrap_or_default()),
            font_size: font_size.unwrap_or(20.0),
            components: vec![],
        };

        initial.components.push(Box::new(TextBox::new(
            "This is a very long text that I wish to wrap and test that it works. In other words, this is but a humble placeholder for what is yet to be implemented the greatest typing speed test written in rust!".to_string(),
            Style {
                font_size:initial.font_size,
                border_size: Some(2.0),
                x: Value::Relative(Box::new(|| (0.5 * window::screen_width()) / 2.0)),
                y: Value::Relative(Box::new(|| (window::screen_height() - 100.0) / 2.0)),
                width: Value::Relative(Box::new(|| window::screen_width() / 2.0)),
                height: Value::Absolute(100.0),
                theme: Theme {
                    bg: Rc::clone(&initial.theme.bg),
                    ghost: Rc::clone(&initial.theme.ghost),
                    text: Rc::clone(&initial.theme.ghost),
                    error: Rc::clone(&initial.theme.error),
                },
                clip: true,
                offset_y: None,
                offset_x: None,
                padding_x: Some(Value::Absolute(10.0)),
                padding_y: Some(Value::Absolute(10.0)),
            }
        )));

        initial
    }

    pub async fn main_loop(&mut self) -> Result<(), Box<dyn Error>> {
        loop {
            if let Some(c) = input::get_char_pressed() {
                match c {
                    'a' => self.theme.set_atom(),
                    'c' => self.theme.set_catppuccin(),
                    _ => (),
                }
            }

            window::clear_background(*self.theme.bg.borrow());

            for comp in &self.components {
                comp.update();
            }

            window::next_frame().await;
        }
    }
}
