use std::rc::Rc;

use macroquad::text::{self, Font};

use crate::app::style::*;
use crate::app::text::PrintOptions;
use crate::app::theme::{Theme, ThemeName};
use crate::app::{self, Value};

pub struct Button {
    pub text: String,
    pub theme_name: ThemeName,
    pub style: Style,
    font: Rc<Font>,
}

impl Button {
    pub fn new(theme_name: ThemeName, style: &Style, font: Rc<Font>) -> Self {
        let theme = Theme::get_theme(&theme_name);

        let f1 = Rc::clone(&style.font_size);
        let f2 = Rc::clone(&style.font_size);
        let f3 = Rc::clone(&style.font_size);

        let tn = theme_name.clone();

        let font1 = Rc::clone(&font);
        let font2 = Rc::clone(&font);

        Button {
            font,
            theme_name: theme_name.clone(),
            text: format!("{:?}", theme_name)
                .split("::")
                .last()
                .unwrap()
                .to_string(),
            style: Style {
                border: Some(BorderParams {
                    size: 2.0,
                    color: Rc::clone(&theme.ghost),
                }),
                theme,
                width: Value::Relative(Box::new(move |_| {
                    text::measure_text(
                        format!("{:?}", tn).split("::").last().unwrap(),
                        Some(&font1),
                        *f2.borrow() as u16,
                        1.0,
                    )
                    .width
                        + 20.0
                })),
                height: Value::Relative(Box::new(move |_| {
                    text::measure_text(
                        format!("{:?}", theme_name).split("::").last().unwrap(),
                        Some(&font2),
                        *f3.borrow() as u16,
                        1.0,
                    )
                    .height
                        + 20.0
                })),
                padding_x: Some(Value::Absolute(10.0)),
                padding_y: Some(Value::Absolute(10.0)),
                font_size: f1,
                ..Style::default()
            },
        }
    }

    pub fn update(&self) {
        self.style.draw_bg();
        self.style.draw_border();
        app::text::print_text(
            &self.style,
            &self.text,
            PrintOptions {
                font: Some(Rc::clone(&self.font)),
                ..PrintOptions::default()
            },
        );
    }
}
