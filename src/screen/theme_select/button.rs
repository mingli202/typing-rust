use std::rc::Rc;

use macroquad::text;

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

        let f1 = Rc::clone(&style.font_size);
        let f2 = Rc::clone(&style.font_size);
        let f3 = Rc::clone(&style.font_size);

        let tn = theme_name.clone();

        Button {
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
                width: Value::Relative(Box::new(move || {
                    text::measure_text(
                        format!("{:?}", tn).split("::").last().unwrap(),
                        None,
                        *f2.borrow() as u16,
                        1.0,
                    )
                    .width
                        + 20.0
                })),
                height: Value::Relative(Box::new(move || {
                    text::measure_text(
                        format!("{:?}", theme_name).split("::").last().unwrap(),
                        None,
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
        screen::text::print_text(
            &self.style,
            &self.text,
            self.style.x.get(),
            self.style.y.get(),
        );
    }
}
