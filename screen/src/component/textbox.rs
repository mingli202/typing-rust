use std::rc::Rc;

use macroquad::text::{self, TextDimensions};

use crate::theme::Theme;

use super::{Component, Shape};

pub struct TextBox {
    text: String,
    font_size: f32,
    color: Rc<Theme>,
    shape: Shape,
}

impl TextBox {
    pub fn new(
        text: String,
        font_size: f32,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        color: Rc<Theme>,
    ) -> TextBox {
        TextBox {
            text,
            font_size,
            shape: Shape {
                x,
                y,
                width,
                height,
            },
            color,
        }
    }

    fn print_text_wrap(&self) {
        let mut size = 0.0;
        let mut i = -1;
        let mut k: i32 = 0;
        let mut last = i;
        let mut lines = 0;

        for word in self.text.split(' ') {
            i += word.len() as i32 + 1;

            let TextDimensions { width, .. } = text::measure_text(
                &self.text[k as usize..i as usize],
                None,
                self.font_size as u16,
                1.0,
            );

            size += width;

            let y = self.shape.y + lines as f32 * self.font_size;
            if y > self.shape.y + self.shape.height {
                return;
            }

            if size > self.shape.width {
                text::draw_text(
                    &self.text[k as usize..last as usize],
                    self.shape.x,
                    y,
                    self.font_size,
                    self.color.text,
                );

                lines += 1;
                k = last + 1;

                size = text::measure_text(
                    &self.text[k as usize..i as usize],
                    None,
                    self.font_size as u16,
                    1.0,
                )
                .width;
            }

            last = i;
        }

        text::draw_text(
            &self.text[k as usize..last as usize],
            self.shape.x,
            self.shape.y + lines as f32 * self.font_size,
            self.font_size,
            self.color.text,
        );
    }
}

impl Component for TextBox {
    fn update(&self) {
        self.print_text_wrap();
    }
}
