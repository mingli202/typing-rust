use macroquad::text::{self, TextParams};

use crate::app::bombparty::Style;

use super::Component;

pub struct Text {
    pub text: String,
    pub style: Style,
}

impl Component for Text {
    fn get_style_mut(&mut self) -> &mut Style {
        &mut self.style
    }
    fn get_style(&self) -> &Style {
        &self.style
    }
    fn refresh(&mut self) {
        text::draw_text_ex(
            &self.text[..],
            self.style.x,
            self.style.y,
            TextParams {
                color: *self.style.theme.text.borrow(),
                font: self.style.font.as_deref(),
                font_size: *self.style.font_size.borrow() as u16,
                ..TextParams::default()
            },
        );
    }
}
