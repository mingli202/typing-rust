#![allow(unused)]

use std::rc::Rc;

use macroquad::color::Color;
use macroquad::shapes;
use macroquad::text::{self, Font, TextDimensions, TextParams};

use super::{Letter, Style};

#[derive(Default)]
pub struct PrintOptions {
    pub x: Option<f32>,
    pub y: Option<f32>,
    pub font: Option<Rc<Font>>,
    pub font_size: Option<f32>,
    pub font_scale: Option<f32>,
    pub color: Option<Color>,
}

pub fn print_text(style: &Style, text: &str, opts: PrintOptions) -> TextDimensions {
    let p_x = style.padding_x();
    let o_x = style.offset_x();
    let p_y = style.padding_y();
    let o_y = style.offset_y();

    let x = opts.x.unwrap_or(style.x.get(style));
    let y = opts.y.unwrap_or(style.y.get(style));

    let fsize = opts.font_size.unwrap_or(*style.font_size.borrow());
    let fscale = opts.font_scale.unwrap_or(1.0);
    let color = opts.color.unwrap_or(*style.theme.text.borrow());

    if style.wrap {
        let wt = WrappedText::new(
            text,
            style.width.get(style) - 2.0 * p_x,
            *style.font_size.borrow(),
            fscale,
            Rc::clone(&opts.font.unwrap()),
        );

        wt.print(x + p_x + o_x, y + p_y + o_y, color);

        TextDimensions {
            width: wt.width,
            height: wt.height,
            offset_y: 0.0,
        }
    } else {
        let TextDimensions { offset_y, .. } =
            macroquad::text::measure_text(text, opts.font.as_deref(), fsize as u16, fscale);

        text::draw_text_ex(
            text,
            x + p_x + o_x,
            y + p_y + o_y + offset_y,
            TextParams {
                font: opts.font.as_deref(),
                font_size: fsize as u16,
                font_scale: fscale,
                color,
                ..TextParams::default()
            },
        )
    }
}

pub struct WrappedText {
    text: Vec<String>,
    height: f32,
    width: f32,
    max_width: f32,
    font_size: f32,
    font_scale: f32,
    font: Rc<Font>,
}

impl WrappedText {
    pub fn new(
        text: &str,
        max_width: f32,
        font_size: f32,
        font_scale: f32,
        font: Rc<Font>,
    ) -> Self {
        let mut lines: Vec<Vec<String>> = vec![];

        let mut line: Vec<String> = vec![];

        let mut w = 0.0;

        for word in text.split_whitespace() {
            let l = line.join(" ") + " " + word;

            let TextDimensions { width, .. } =
                text::measure_text(&l, Some(&font), font_size as u16, font_scale);

            if width > max_width {
                let _w = text::measure_text(
                    &line.join(" ")[..],
                    Some(&font),
                    font_size as u16,
                    font_scale,
                )
                .width;

                if _w > w {
                    w = _w;
                }

                lines.push(line.clone());
                line = vec![word.to_string()];
            } else {
                line.push(word.to_string());
            }
        }
        lines.push(line);

        let text = lines.iter().map(|l| l.join(" ")).collect::<Vec<String>>();

        let last = text.last().unwrap();
        let TextDimensions { height: h_last, .. } =
            text::measure_text(last, Some(&font), font_size as u16, 1.0);
        let height = (text.len() - 1) as f32 * font_size + h_last;

        WrappedText {
            height,
            text,
            max_width,
            width: w,
            font_size,
            font,
            font_scale,
        }
    }

    pub fn height(&self) -> f32 {
        self.height
    }
    pub fn width(&self) -> f32 {
        self.width
    }
    pub fn max_width(&self) -> f32 {
        self.max_width
    }

    pub fn print(&self, x: f32, y: f32, color: Color) {
        for (i, line) in self.text.iter().enumerate() {
            let TextDimensions { offset_y, .. } = macroquad::text::measure_text(
                &line[..],
                Some(&self.font),
                self.font_size as u16,
                self.font_scale,
            );

            text::draw_text_ex(
                &line[..],
                x,
                y + offset_y + i as f32 * self.font_size,
                TextParams {
                    font: Some(&self.font),
                    font_size: self.font_size as u16,
                    font_scale: self.font_scale,
                    color,
                    ..TextParams::default()
                },
            );
        }
    }
}
