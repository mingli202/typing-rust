use macroquad::text::{self, TextDimensions, TextParams};

use crate::app::bombparty::Style;

use super::Component;

pub struct Text {
    pub text: String,
    pub style: Style,
    lines: Vec<(String, f32, f32)>,
    offset_y: f32,
}

impl Text {
    pub fn new(style: Style, text: String) -> Self {
        Text {
            style,
            text,
            lines: vec![],
            offset_y: 0.0,
        }
    }
}

impl Component for Text {
    fn build(&mut self) {
        let txt = self.text.replace("\t", "    ");
        let lines = txt.split("\n");

        let mut max_width = 0.0;
        let mut height = 0.0;

        let fsize = *self.style.font_size.borrow();

        let mut liness = vec![];

        self.offset_y = text::measure_text(
            "q",
            self.style.font.as_deref(),
            *self.style.font_size.borrow() as u16,
            1.0,
        )
        .height;

        for line in lines {
            let TextDimensions { width: w, .. } = text::measure_text(
                line,
                self.style.font.as_deref(),
                *self.style.font_size.borrow() as u16,
                1.0,
            );

            liness.push((line.to_string(), 0.0, height + self.offset_y));

            height += fsize;

            if w > max_width {
                max_width = w;
            }
        }

        self.style.width = max_width;
        self.style.height = height;

        self.lines = liness;
    }
    fn get_style_mut(&mut self) -> &mut Style {
        &mut self.style
    }
    fn get_style(&self) -> &Style {
        &self.style
    }
    fn refresh(&mut self) {
        for line in &self.lines {
            text::draw_text_ex(
                &line.0,
                line.1 + self.style.x,
                line.2 + self.style.y,
                TextParams {
                    color: *self.style.theme.text.borrow(),
                    font: self.style.font.as_deref(),
                    font_size: *self.style.font_size.borrow() as u16,
                    ..TextParams::default()
                },
            );
        }
    }
}
