use std::rc::Rc;

mod textbox;
use macroquad::text::Font;
pub use textbox::TextBox;

use crate::theme::Theme;

pub trait Component {
    /// Function that will be called on each frame
    fn update(&self);
}

pub enum Value<T> {
    Relative(Box<dyn Fn() -> T>),
    Absolute(T),
}

impl<T: Clone> Value<T> {
    fn get(&self) -> T {
        match self {
            Self::Absolute(v) => v.clone(),
            Self::Relative(v) => v(),
        }
    }
}

pub struct Style {
    pub x: Value<f32>,
    pub y: Value<f32>,
    pub width: Value<f32>,
    pub height: Value<f32>,
    pub font_size: f32,
    pub theme: Rc<Theme>,
    pub border_size: Option<f32>,
}
