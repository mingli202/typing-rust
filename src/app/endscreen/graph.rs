use std::process;
use std::rc::Rc;
use std::time::Duration;

use macroquad::color::Color;
use macroquad::text::{self, Font};
use macroquad::{shapes, window};

use crate::app::style::Style;
use crate::app::text::PrintOptions;
use crate::app::theme::Theme;
use crate::app::{self, util, Value};

pub struct Graph {
    incremental_wpm: Vec<(Duration, u16)>,
    max_wpm: u16,
    time: Duration,
    wrongs: i32,
    pub style: Style,
    font: Rc<Font>,
}

impl Graph {
    pub fn new(
        style: &Style,
        incremental_wpm: Vec<(Duration, u16)>,
        max_wpm: u16,
        time: Duration,
        wrongs: i32,
        font: Rc<Font>,
    ) -> Self {
        Graph {
            max_wpm,
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
            2.0,
            Color::new(0.0, 1.0, 0.0, 1.0),
        );

        let x = self.style.x();
        let y = self.style.y();
        let fsize = *self.style.font_size.borrow();

        let wpm_range = Self::range(0.0, self.max_wpm as f32, 5);
        let wpm_y_range = Self::range(y, y + self.style.height() - 2.0 * fsize, 5);

        for (_wpm, _y) in wpm_range.iter().rev().zip(wpm_y_range) {
            app::text::print_text(
                &self.style,
                &format!("{}", _wpm),
                PrintOptions {
                    x: Some(x),
                    y: Some(_y),
                    font: Some(Rc::clone(&self.font)),
                    font_scale: Some(1.0),
                    color: Some(*self.style.theme.ghost.borrow()),
                    ..PrintOptions::default()
                },
            );
        }
    }

    fn range(start: f32, end: f32, n_steps: usize) -> Vec<f32> {
        let mut o = vec![];

        for i in 0..=n_steps {
            o.push(start + (end - start) * i as f32 / n_steps as f32);
        }

        o
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn range_1() {
        let arr = Graph::range(1.0, 4.0, 3);
        assert_eq!(arr, vec![1.0, 2.0, 3.0, 4.0]);
    }
    #[test]
    fn range_2() {
        let arr = Graph::range(100.0, 400.0, 3);
        assert_eq!(arr, vec![100.0, 200.0, 300.0, 400.0]);
    }
}
