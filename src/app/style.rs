use macroquad::{color::Color, shapes, window};
use std::cell::RefCell;
use std::rc::Rc;

use crate::app::theme::Theme;

pub struct BorderParams {
    pub size: f32,
    pub color: Rc<RefCell<Color>>,
}

#[derive(Default)]
pub struct Style {
    pub x: Value<f32>,
    pub y: Value<f32>,
    pub width: Value<f32>,
    pub height: Value<f32>,
    pub font_size: Rc<RefCell<f32>>,
    pub theme: Theme,
    pub border: Option<BorderParams>,
    pub clip: bool,
    pub offset_x: Option<Value<f32>>,
    pub offset_y: Option<Value<f32>>,
    pub padding_x: Option<Value<f32>>,
    pub padding_y: Option<Value<f32>>,
    pub wrap: bool,
    pub parent: Option<Rc<Style>>,
}

impl Style {
    pub fn draw_border(&self) {
        if let Some(border) = &self.border {
            shapes::draw_rectangle_lines(
                self.x.get(None),
                self.y.get(None),
                self.width.get(None),
                self.height.get(None),
                border.size,
                *border.color.borrow(),
            );
        }
    }

    pub fn draw_bg(&self) {
        shapes::draw_rectangle(
            self.x.get(None),
            self.y.get(None),
            self.width.get(None),
            self.height.get(None),
            *self.theme.bg.borrow(),
        );
    }

    pub fn draw_mask(&self) {
        if self.clip {
            let Self {
                x,
                y,
                width,
                height,
                theme,
                padding_y,
                ..
            } = self;

            let p_y = match padding_y {
                Some(p) => p.get(None),
                _ => 0.0,
            };

            let color = *theme.bg.borrow();

            // top
            shapes::draw_rectangle(x.get(None), 0.0, width.get(None), y.get(None) + p_y, color);

            // bottom
            shapes::draw_rectangle(
                x.get(None),
                y.get(None) + height.get(None) - p_y,
                width.get(None),
                window::screen_height() - (y.get(None) + height.get(None) - p_y),
                color,
            );
        }
    }
}

pub enum Value<T> {
    Relative(Box<dyn Fn(Option<&Style>) -> T>),
    Absolute(T),
}

impl<T: Clone> Value<T> {
    pub fn get(&self, style: Option<&Style>) -> T {
        match self {
            Self::Absolute(v) => v.clone(),
            Self::Relative(v) => v(style),
        }
    }
}

impl<T: Default> Default for Value<T> {
    fn default() -> Self {
        Value::Absolute(T::default())
    }
}
