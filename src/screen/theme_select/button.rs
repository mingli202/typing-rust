use std::rc::Rc;

use macroquad::text::{self, TextDimensions};

use crate::screen::style::{BorderParams, Style};
use crate::screen::theme::{Theme, ThemeName};
use crate::screen::{self, Value};

pub struct Button {
    pub text: String,
    pub theme_name: ThemeName,
    pub style: Style,
}

impl Button {
    pub fn new(theme_name: ThemeName, style: &Style) -> Self {
        let theme = Theme::get_theme(&theme_name);

        let text = format!("{:?}", theme_name)
            .split("::")
            .last()
            .unwrap()
            .to_string();

        let TextDimensions {
            width,
            offset_y,
            height,
        } = text::measure_text(&text, None, style.font_size as u16, 1.0);

        Button {
            theme_name,
            text,
            style: Style {
                border: Some(BorderParams {
                    size: 2.0,
                    color: Rc::clone(&theme.ghost),
                }),
                theme,
                width: Value::Absolute(width + 20.0),
                height: Value::Absolute(height + 20.0),
                offset_y: Some(Value::Absolute(offset_y)),
                padding_x: Some(Value::Absolute(10.0)),
                padding_y: Some(Value::Absolute(10.0)),
                font_size: style.font_size,
                ..Style::default()
            },
        }
    }

    pub fn update(&self) {
        self.style.draw_bg();
        self.style.draw_border();
        screen::text::print_text(
            &self.style,
            &self.text,
            self.style.x.get(),
            self.style.y.get(),
        );
    }
}
