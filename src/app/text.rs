#![allow(unused)]

use macroquad::color::Color;
use macroquad::shapes;
use macroquad::text::{self, TextDimensions, TextParams};

use super::{Letter, Style};

pub fn print_text(style: &Style, text: &str, x: f32, y: f32) {
    let p_x = match &style.padding_x {
        Some(p) => p.get(style),
        _ => 0.0,
    };

    let o_x = match &style.offset_x {
        Some(p) => p.get(style),
        _ => 0.0,
    };

    let p_y = match &style.padding_y {
        Some(p) => p.get(style),
        _ => 0.0,
    };

    let o_y = match &style.offset_y {
        Some(p) => p.get(style),
        _ => 0.0,
    };

    let fsize = *style.font_size.borrow();
    if style.wrap {
        let wt = WrappedText::new(
            text,
            style.width.get(style),
            TextParams {
                font_size: fsize as u16,
                ..TextParams::default()
            },
        );

        wt.print(style, x, y);
    } else {
        let TextDimensions { offset_y, .. } =
            macroquad::text::measure_text(text, None, fsize as u16, 1.0);

        text::draw_text(
            text,
            x + p_x + o_x,
            y + p_y + o_y + offset_y,
            fsize,
            *style.theme.text.borrow(),
        );
    }
}

pub struct WrappedText<'a> {
    text: Vec<String>,
    height: f32,
    width: f32,
    max_width: f32,
    text_params: TextParams<'a>,
}

impl<'a> WrappedText<'a> {
    pub fn new(text: &str, max_width: f32, text_params: TextParams<'a>) -> Self {
        let mut lines: Vec<Vec<String>> = vec![];

        let mut line: Vec<String> = vec![];

        let mut w = 0.0;

        for word in text.split_whitespace() {
            let l = line.join(" ") + " " + word;

            let TextDimensions { width, .. } =
                text::measure_text(&l, None, text_params.font_size, text_params.font_scale);

            if width > max_width {
                let _w = text::measure_text(
                    &line.join(" ")[..],
                    None,
                    text_params.font_size,
                    text_params.font_scale,
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
            text::measure_text(last, None, text_params.font_size, text_params.font_scale);
        let height = (text.len() - 1) as f32 * text_params.font_size as f32 + h_last;

        WrappedText {
            height,
            text,
            max_width,
            width: w,
            text_params,
        }
    }

    pub fn get_height(&self) -> f32 {
        self.height
    }
    pub fn get_width(&self) -> f32 {
        self.width
    }
    pub fn get_max_width(&self) -> f32 {
        self.max_width
    }

    pub fn print(&self, style: &Style, x: f32, y: f32) {
        let p_x = match &style.padding_x {
            Some(p) => p.get(style),
            _ => 0.0,
        };

        let o_x = match &style.offset_x {
            Some(p) => p.get(style),
            _ => 0.0,
        };

        let p_y = match &style.padding_y {
            Some(p) => p.get(style),
            _ => 0.0,
        };

        let o_y = match &style.offset_y {
            Some(p) => p.get(style),
            _ => 0.0,
        };

        for (i, line) in self.text.iter().enumerate() {
            let TextDimensions { offset_y, .. } =
                macroquad::text::measure_text(&line[..], None, self.text_params.font_size, 1.0);

            text::draw_text(
                &line[..],
                x + p_x + o_x,
                y + p_y + o_y + offset_y + i as f32 * self.text_params.font_size as f32,
                *style.font_size.borrow(),
                *style.theme.text.borrow(),
            );
        }
    }
}
