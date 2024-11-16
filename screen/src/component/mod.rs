use macroquad::{color::Color, shapes, window};
use std::cell::RefCell;
use std::rc::Rc;

pub mod textbox;

use crate::theme::Theme;

pub trait Component {
    /// Function that will be called on each frame
    fn update(&self);
    fn onclick(&self);
}

pub enum Value<T> {
    Relative(Box<dyn Fn() -> T>),
    Absolute(T),
}

impl<T: Clone> Value<T> {
    pub fn get(&self) -> T {
        match self {
            Self::Absolute(v) => v.clone(),
            Self::Relative(v) => v(),
        }
    }
}

pub struct BorderParams {
    pub size: f32,
    pub color: Rc<RefCell<Color>>,
}

pub struct Style {
    pub x: Value<f32>,
    pub y: Value<f32>,
    pub width: Value<f32>,
    pub height: Value<f32>,
    pub font_size: f32,
    pub theme: Theme,
    pub border: Option<BorderParams>,
    pub clip: bool,
    pub offset_x: Option<Value<f32>>,
    pub offset_y: Option<Value<f32>>,
    pub padding_x: Option<Value<f32>>,
    pub padding_y: Option<Value<f32>>,
}

impl Default for Style {
    fn default() -> Self {
        Style {
            x: Value::Absolute(0.0),
            y: Value::Absolute(0.0),
            width: Value::Absolute(0.0),
            height: Value::Absolute(0.0),
            font_size: 0.0,
            theme: Theme::default(),
            border: None,
            clip: false,
            offset_x: None,
            offset_y: None,
            padding_x: None,
            padding_y: None,
        }
    }
}

impl Style {
    pub fn draw_border(&self) {
        if let Some(border) = &self.border {
            shapes::draw_rectangle_lines(
                self.x.get(),
                self.y.get(),
                self.width.get(),
                self.height.get(),
                border.size,
                *border.color.borrow(),
            );
        }
    }

    pub fn draw_bg(&self) {
        shapes::draw_rectangle(
            self.x.get(),
            self.y.get(),
            self.width.get(),
            self.height.get(),
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
                padding_x,
                ..
            } = self;

            let p_x = match padding_x {
                Some(p) => p.get(),
                _ => 0.0,
            };

            let p_y = match padding_y {
                Some(p) => p.get(),
                _ => 0.0,
            };

            let color = *theme.bg.borrow();

            // left mask
            shapes::draw_rectangle(0.0, 0.0, x.get() + p_x, window::screen_height(), color);

            // right mask
            shapes::draw_rectangle(
                x.get() + width.get() - p_x,
                0.0,
                window::screen_width() - (width.get() + x.get() - p_x),
                window::screen_height(),
                color,
            );

            // top
            shapes::draw_rectangle(x.get(), 0.0, width.get(), y.get() + p_y, color);

            // bottom
            shapes::draw_rectangle(
                x.get(),
                y.get() + height.get() - p_y,
                width.get(),
                window::screen_height() - (y.get() + height.get() - p_y),
                color,
            );
        }
    }
}
