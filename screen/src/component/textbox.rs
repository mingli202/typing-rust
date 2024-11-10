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

    pub fn print_text(&self, text: &str, x: f32, y: f32) {
        let TextDimensions { offset_y, .. } =
            text::measure_text(text, None, self.style.font_size as u16, 1.0);

        let p_x = match &self.style.padding_x {
            Some(p) => p.get(),
            _ => 0.0,
        };

        let o_x = match &self.style.offset_x {
            Some(p) => p.get(),
            _ => 0.0,
        };

        let p_y = match &self.style.padding_y {
            Some(p) => p.get(),
            _ => 0.0,
        };

        let o_y = match &self.style.offset_y {
            Some(p) => p.get(),
            _ => 0.0,
        };

        text::draw_text(
            text,
            x + p_x + o_x,
            y + p_y + o_y + offset_y,
            self.style.font_size,
            *self.style.theme.text.borrow(),
        );
    }

    fn print_text_wrap(&self) {
        let mut i = -1;
        let mut k: i32 = 0;
        let mut last = i;
        let mut lines = 0;

        let p_x = match &self.style.padding_x {
            Some(p) => p.get(),
            _ => 0.0,
        };

        let p_y = match &self.style.padding_y {
            Some(p) => p.get(),
            _ => 0.0,
        };

        for word in self.text.split(' ') {
            i += word.len() as i32 + 1;

            let y = self.style.y.get() + lines as f32 * self.style.font_size;
            if y > self.style.y.get() + self.style.height.get() - p_y {
                return;
            }

            let TextDimensions { width, .. } = text::measure_text(
                &self.text[k as usize..i as usize],
                None,
                self.style.font_size as u16,
                1.0,
            );

            if width > self.style.width.get() - 2.0 * p_x {
                self.print_text(&self.text[k as usize..last as usize], self.style.x.get(), y);

                lines += 1;
                k = last + 1;
            }

            last = i;
        }

        let y = self.style.y.get() + lines as f32 * self.style.font_size;

        self.print_text(&self.text[k as usize..last as usize], self.style.x.get(), y);
    }
}

impl Component for TextBox {
    fn update(&self) {
        self.style.draw_bg();
        self.print_text_wrap();
        self.style.draw_mask();
        self.style.draw_border();
    }
}
