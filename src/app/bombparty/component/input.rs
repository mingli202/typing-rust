use macroquad::text::{TextDimensions, TextParams};
use macroquad::{input, shapes, text};

use crate::app::bombparty::style::Style;

use super::Component;

pub struct Input {
    style: Style,
    value: String,
    focused: bool,
}

impl Input {
    pub fn new(style: Style) -> Self {
        let initial_value = "asdf".to_string();

        let TextDimensions {
            width,
            height,
            offset_y,
        } = text::measure_text(
            &initial_value[..],
            style.font.as_deref(),
            *style.font_size.borrow() as u16,
            1.0,
        );

        Input {
            style: Style {
                width,
                height: height + offset_y,
                ..style
            },
            value: initial_value,
            focused: false,
        }
    }

    pub fn get_value(&self) -> &String {
        &self.value
    }
}

impl Component for Input {
    fn on_click_in(&mut self) {
        self.focused = true;
    }
    fn on_click_out(&mut self) {
        self.focused = false;
    }

    fn refresh(&mut self) {
        if self.focused {
            if let Some(c) = input::get_char_pressed() {
                self.value += &c.to_string();
            }
        }

        let TextDimensions {
            width,
            height,
            offset_y,
        } = text::measure_text(
            &self.value[..],
            self.style.font.as_deref(),
            *self.style.font_size.borrow() as u16,
            1.0,
        );

        self.style.width = width;
        self.style.height = height;

        text::draw_text_ex(
            &self.value[..],
            self.style.x,
            self.style.y + offset_y,
            TextParams {
                color: *self.style.theme.text.borrow(),
                font: self.style.font.as_deref(),
                font_size: *self.style.font_size.borrow() as u16,
                ..TextParams::default()
            },
        );

        shapes::draw_rectangle_lines(
            self.style.x,
            self.style.y,
            self.style.width,
            self.style.height,
            2.0,
            if self.focused {
                *self.style.theme.text.borrow()
            } else {
                *self.style.theme.ghost.borrow()
            },
        );
    }

    fn get_style(&self) -> &Style {
        &self.style
    }
    fn get_style_mut(&mut self) -> &mut Style {
        &mut self.style
    }
}
