use std::rc::Rc;
use std::time::Duration;

use macroquad::color::Color;
use macroquad::text::Font;
use macroquad::{shapes, window};

use crate::app::style::Style;
use crate::app::theme::Theme;
use crate::app::{util, Value};

pub struct Graph {
    incremental_wpm: Vec<u16>,
    time: Duration,
    wrongs: i32,
    pub style: Style,
    font: Rc<Font>,
}

impl Graph {
    pub fn new(
        style: &Style,
        incremental_wpm: Vec<u16>,
        time: Duration,
        wrongs: i32,
        font: Rc<Font>,
    ) -> Self {
        Graph {
            font,
            incremental_wpm,
            time,
            wrongs,
            style: Style {
                theme: Theme {
                    text: Rc::clone(&style.theme.text),
                    ghost: Rc::clone(&style.theme.ghost),
                    error: Rc::clone(&style.theme.error),
                    bg: Rc::clone(&style.theme.bg),
                },
                font_size: Rc::clone(&style.font_size),
                x: Value::Relative(Box::new(|_| {
                    util::clamp(0.0, window::screen_width() / 10.0, 30.0)
                })),
                y: Value::Relative(Box::new(|_| {
                    util::clamp(10.0, window::screen_height() / 10.0, 30.0)
                })),
                width: Value::Relative(Box::new(|this| {
                    window::screen_width() - 2.0 * this.x.get(this)
                })),
                height: Value::Relative(Box::new(|this| {
                    window::screen_height() / 2.0 - 2.0 * this.y.get(this)
                })),
                ..Style::default()
            },
        }
    }

    pub fn update(&self, parent: &Style) {
        shapes::draw_rectangle_lines(
            self.style.x.get(parent),
            self.style.y.get(parent),
            self.style.width.get(&self.style),
            self.style.height.get(&self.style),
            1.0,
            Color::new(0.0, 1.0, 0.0, 1.0),
        );
    }
}
