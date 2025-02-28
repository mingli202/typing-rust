use std::rc::Rc;
use std::time::Duration;

use macroquad::text::{self, Font};
use macroquad::window;

use crate::app::text::PrintOptions;
use crate::app::{theme::Theme, Style, Value};

pub struct Wpm {
    pub style: Style,
    font: Rc<Font>,
    wpm: u16,
    accuracy: i32,
    time: Duration,
}

impl Wpm {
    pub fn new(style: &Style, wpm: u16, accuracy: i32, time: Duration, font: Rc<Font>) -> Wpm {
        Wpm {
            font,
            wpm,
            accuracy,
            time,
            style: Style {
                y: Value::Relative(Box::new(move |_| window::screen_height() / 2.0)),
                font_size: Rc::clone(&style.font_size),
                theme: Theme {
                    bg: Rc::clone(&style.theme.bg),
                    ghost: Rc::clone(&style.theme.ghost),
                    text: Rc::clone(&style.theme.text),
                    error: Rc::clone(&style.theme.error),
                },
                ..Style::default()
            },
        }
    }

    pub fn update(&self) {
        let stats = [
            ("WPM", format!("{}", self.wpm)),
            ("ACC", format!("{}%", self.accuracy)),
            (
                "TIME",
                format!("{:.1}s", self.time.as_millis() as f32 / 1000.0),
            ),
        ];

        let (xs, width) = self.measure_text(&stats);
        let margin_x = (window::screen_width() - width) / 2.0;

        let fsize = *self.style.font_size.borrow();
        let fscale = 0.65;

        for ((key, value), x) in stats.iter().zip(xs) {
            crate::app::text::print_text(
                &self.style,
                key,
                PrintOptions {
                    font: Some(Rc::clone(&self.font)),
                    font_scale: Some(fscale),
                    x: Some(x + margin_x),
                    color: Some(*self.style.theme.ghost.borrow()),
                    ..PrintOptions::default()
                },
            );

            crate::app::text::print_text(
                &self.style,
                value,
                PrintOptions {
                    font: Some(Rc::clone(&self.font)),
                    x: Some(x + margin_x),
                    y: Some(self.style.y() + fsize),
                    ..PrintOptions::default()
                },
            );
        }
    }

    fn measure_text(&self, stats: &[(&str, String)]) -> (Vec<f32>, f32) {
        let mut x = 0.0;

        let fsize = *self.style.font_size.borrow();
        let fscale = 0.65;

        let mut xs = Vec::with_capacity(3);

        for (i, (key, value)) in stats.iter().enumerate() {
            xs.push(x);
            let w1 = text::measure_text(key, Some(&self.font), fsize as u16, fscale).width;
            let w2 = text::measure_text(value, Some(&self.font), fsize as u16, 1.0).width;

            x += if w1 > w2 { w1 } else { w2 };
            if i != stats.len() - 1 {
                x += text::measure_text("    ", Some(&self.font), fsize as u16, 1.0).width
            }
        }

        (xs, x)
    }
}
