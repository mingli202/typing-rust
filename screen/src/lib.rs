use macroquad::{shapes, window};
use std::error::Error;
use std::rc::Rc;

use data_provider::Data;

mod theme;
use theme::Theme;

mod component;
use component::TextBox;
use component::{Component, Value};

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

pub struct Screen {
    font_size: f32,
    theme: Rc<Theme>,
    mode: Mode,
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
            theme: Rc::new(theme.unwrap_or_default()),
            mode: mode.unwrap_or_default(),
            font_size: font_size.unwrap_or(20.0),
            components: vec![],
        };

        initial.components.push(Box::new(TextBox::new(
            "this is a very long string that I want to wrap and test that its work hopefully so that I don't have to do random shit again".to_string(),
            initial.font_size,
            Value::Relative(Box::new(|| (0.5 * window::screen_width()) / 2.0)),
            Value::Relative(Box::new(|| (window::screen_height() - 100.0) / 2.0)),
            Value::Relative(Box::new(|| window::screen_width() / 2.0)),
            Value::Absolute(100.0),
            Rc::clone(&initial.theme),
        )));

        initial
    }

    pub async fn main_loop(&self) -> Result<(), Box<dyn Error>> {
        loop {
            window::clear_background(self.theme.bg);

            for comp in &self.components {
                comp.update();
            }

            window::next_frame().await;
        }
    }
}
