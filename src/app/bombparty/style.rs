use std::cell::RefCell;
use std::rc::Rc;

use macroquad::text::Font;

use crate::app::theme::Theme;
use crate::app::{self, BorderParams};

#[derive(Debug, Clone, Default)]
pub struct Style {
    pub theme: Theme,
    pub font: Option<Rc<Font>>,
    pub font_size: Rc<RefCell<f32>>,
    pub border: Option<BorderParams>,
    pub clip: bool,
    pub wrap: bool,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Style {
    pub fn from(style: &app::Style) -> Self {
        Style {
            theme: style.copy_theme(),
            font: style.font.as_ref().map(Rc::clone),
            font_size: Rc::clone(&style.font_size),
            ..Self::default()
        }
    }
}
