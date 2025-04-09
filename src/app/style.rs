use macroquad::text::Font;
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
    pub font: Option<Rc<Font>>,
}

impl Style {
    pub fn draw_border(&self) {
        if let Some(border) = &self.border {
            shapes::draw_rectangle_lines(
                self.x.get(self),
                self.y.get(self),
                self.width.get(self),
                self.height.get(self),
                border.size,
                *border.color.borrow(),
            );
        }
    }

    pub fn draw_bg(&self) {
        shapes::draw_rectangle(
            self.x.get(self),
            self.y.get(self),
            self.width.get(self),
            self.height.get(self),
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
                Some(p) => p.get(self),
                _ => 0.0,
            };

            let color = *theme.bg.borrow();

            // top
            shapes::draw_rectangle(x.get(self), 0.0, width.get(self), y.get(self) + p_y, color);

            // bottom
            shapes::draw_rectangle(
                x.get(self),
                y.get(self) + height.get(self) - p_y,
                width.get(self),
                window::screen_height() - (y.get(self) + height.get(self) - p_y),
                color,
            );
        }
    }

    pub fn x(&self) -> f32 {
        self.x.get(self)
    }
    pub fn y(&self) -> f32 {
        self.y.get(self)
    }
    pub fn width(&self) -> f32 {
        self.width.get(self)
    }
    pub fn height(&self) -> f32 {
        self.height.get(self)
    }
    pub fn padding_x(&self) -> f32 {
        if let Some(v) = &self.padding_x {
            v.get(self)
        } else {
            0.0
        }
    }
    pub fn padding_y(&self) -> f32 {
        if let Some(v) = &self.padding_y {
            v.get(self)
        } else {
            0.0
        }
    }
    pub fn offset_x(&self) -> f32 {
        if let Some(v) = &self.offset_x {
            v.get(self)
        } else {
            0.0
        }
    }
    pub fn offset_y(&self) -> f32 {
        if let Some(v) = &self.offset_y {
            v.get(self)
        } else {
            0.0
        }
    }

    pub fn copy(&self) -> Style {
        todo!();
    }
}

pub enum Value<T> {
    Relative(Box<dyn Fn(&Style) -> T>),
    Absolute(T),
}

impl<T: Clone> Value<T> {
    pub fn get(&self, style: &Style) -> T {
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
