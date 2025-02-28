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

    pub fn update(&self) {
        let x = self.style.x();
        let y = self.style.y();
        let width = self.style.width();
        let height = self.style.height();
        let fsize = *self.style.font_size.borrow();

        let width_of_space = text::measure_text(" ", Some(&self.font), fsize as u16, 1.0).width;

        let wpm_range = Self::range(0.0, self.max_wpm as f32, 5);
        let wpm_y_range = Self::range(y, y + height - fsize - width_of_space, 5);

        /*
         * draw vertical label and horizontal lines
         * */
        let mut w = 0.0;
        let mut offset_xs = vec![];

        for _wpm in &wpm_range {
            let _w =
                text::measure_text(&format!("{}", _wpm), Some(&self.font), fsize as u16, 0.8).width;

            offset_xs.push(_w);

            if _w > w {
                w = _w;
            }
        }

        for i in 0..wpm_range.len() {
            let text = format!("{}", wpm_range[wpm_range.len() - 1 - i]);
            let text_height = text::measure_text(&text, Some(&self.font), fsize as u16, 0.8).height;

            app::text::print_text(
                &self.style,
                &text,
                PrintOptions {
                    x: Some(x + w - offset_xs[offset_xs.len() - 1 - i]),
                    y: Some(wpm_y_range[i] - text_height / 2.0),
                    font: Some(Rc::clone(&self.font)),
                    font_scale: Some(0.8),
                    color: Some(*self.style.theme.ghost.borrow()),
                    ..PrintOptions::default()
                },
            );

            shapes::draw_line(
                x + w + width_of_space,
                wpm_y_range[i],
                x + width - width_of_space - w,
                wpm_y_range[i],
                2.0,
                *self.style.theme.ghost.borrow(),
            );
        }

        /*
         * draw horizontal label and vertical lines
         * */
        w += width_of_space;

        let time_x_range: Vec<f32> = (0..=self.time.as_secs())
            .map(|s| (1000.0 * s as f32 / self.time.as_millis() as f32) * (width - 2.0 * w) + x + w)
            .collect();

        let time_range: Vec<u64> = (0..=self.time.as_secs()).collect();

        for (_t, _x) in time_range.iter().zip(time_x_range) {
            let text = format!("{}", _t);
            let text_dim = text::measure_text(&text, Some(&self.font), fsize as u16, 0.8);

            app::text::print_text(
                &self.style,
                &text,
                PrintOptions {
                    x: Some(_x - text_dim.width / 2.0),
                    y: Some(y + height - fsize),
                    font: Some(Rc::clone(&self.font)),
                    font_scale: Some(0.8),
                    color: Some(*self.style.theme.ghost.borrow()),
                    ..PrintOptions::default()
                },
            );

            shapes::draw_line(
                _x,
                y,
                _x,
                y + height - fsize - width_of_space,
                2.0,
                *self.style.theme.ghost.borrow(),
            );
        }

        /*
         * draw the graph
         * */
        let mut last_x = x + w;
        let mut last_y = height - fsize - width_of_space + y;
        for (t, wpm) in self.incremental_wpm.iter() {
            let x_p =
                t.as_millis() as f32 * (width - 2.0 * w) / self.time.as_millis() as f32 + x + w;

            let y_p = (height - fsize - width_of_space)
                - *wpm as f32 * (height - fsize - width_of_space) / self.max_wpm as f32
                + y;

            shapes::draw_circle(x_p, y_p, 4.0, *self.style.theme.text.borrow());

            shapes::draw_line(
                last_x,
                last_y,
                x_p,
                y_p,
                2.0,
                *self.style.theme.text.borrow(),
            );

            last_x = x_p;
            last_y = y_p;
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
