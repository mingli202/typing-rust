use std::rc::Rc;

use macroquad::text::{self, TextDimensions};

use crate::theme::Theme;

use super::{Component, Shape, Value};

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
        x: Value<f32>,
        y: Value<f32>,
        width: Value<f32>,
        height: Value<f32>,
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

            let y = self.shape.y.get() + lines as f32 * self.font_size;
            if y > self.shape.y.get() + self.shape.height.get() {
                return;
            }

            if width > self.shape.width.get() {
                text::draw_text(
                    &self.text[k as usize..last as usize],
                    self.shape.x.get(),
                    y,
                    self.font_size,
                    self.color.text,
                );

                lines += 1;
                k = last + 1;
            }

            last = i;
        }

        text::draw_text(
            &self.text[k as usize..last as usize],
            self.shape.x.get(),
            self.shape.y.get() + lines as f32 * self.font_size,
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
