use macroquad::text::{self, TextDimensions};

use super::{Component, Style};

pub struct TextBox {
    text: String,
    style: Style,
}

impl TextBox {
    pub fn new(text: String, style: Style) -> TextBox {
        TextBox { text, style }
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
                self.style.font_size as u16,
                1.0,
            );

            let y = self.style.y.get() + lines as f32 * self.style.font_size;
            if y > self.style.y.get() + self.style.height.get() {
                return;
            }

            if width > self.style.width.get() {
                text::draw_text(
                    &self.text[k as usize..last as usize],
                    self.style.x.get(),
                    y,
                    self.style.font_size,
                    self.style.theme.text,
                );

                lines += 1;
                k = last + 1;
            }

            last = i;
        }

        text::draw_text(
            &self.text[k as usize..last as usize],
            self.style.x.get(),
            self.style.y.get() + lines as f32 * self.style.font_size,
            self.style.font_size,
            self.style.theme.text,
        );
    }
}

impl Component for TextBox {
    fn update(&self) {
        self.print_text_wrap();
    }
}
