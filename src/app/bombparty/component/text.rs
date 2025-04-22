use macroquad::text::{self, TextDimensions, TextParams};

use crate::app::bombparty::Style;

use super::Component;

pub struct Text {
    pub text: String,
    pub style: Style,
    lines: Vec<(String, f32, f32)>,
}

impl Text {
    pub fn new(mut style: Style, text: String) -> Self {
        let txt = text.replace("\t", "    ");
        let lines = txt.split("\n");

        let mut max_width = 0.0;
        let mut height = 0.0;

        let fsize = *style.font_size.borrow();

        let mut liness = vec![];

        for line in lines {
            let TextDimensions {
                width: w, offset_y, ..
            } = text::measure_text(
                line,
                style.font.as_deref(),
                *style.font_size.borrow() as u16,
                1.0,
            );

            liness.push((line.to_string(), style.x, style.y + height + offset_y));

            height += fsize;

            if w > max_width {
                max_width = w;
            }
        }

        style.width = max_width;
        style.height = height;

        Text {
            style,
            text,
            lines: liness,
        }
    }
}

impl Component for Text {
    fn get_style_mut(&mut self) -> &mut Style {
        &mut self.style
    }
    fn get_style(&self) -> &Style {
        &self.style
    }
    fn refresh(&mut self) {
        let txt = self.text.replace("\t", "    ");
        let lines = txt.split("\n");

        let mut max_width = 0.0;
        let mut height = 0.0;

        let fsize = *self.style.font_size.borrow();

        for line in lines {
            let TextDimensions {
                width: w, offset_y, ..
            } = text::measure_text(
                line,
                self.style.font.as_deref(),
                *self.style.font_size.borrow() as u16,
                1.0,
            );

            text::draw_text_ex(
                line,
                self.style.x,
                self.style.y + height + offset_y,
                TextParams {
                    color: *self.style.theme.text.borrow(),
                    font: self.style.font.as_deref(),
                    font_size: *self.style.font_size.borrow() as u16,
                    ..TextParams::default()
                },
            );

            height += fsize;

            if w > max_width {
                max_width = w;
            }
        }

        self.style.width = max_width;
        self.style.height = height;
    }
}
