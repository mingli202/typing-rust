use std::rc::Rc;
use std::sync::mpsc::Sender;

use macroquad::text::{self, Font, TextDimensions};

use crate::app::text::PrintOptions;
use crate::app::theme::Theme;
use crate::app::{self, BorderParams, Style, Value};

use super::StateAction;

pub trait Component {
    fn onclick(&self);
    fn onhover(&self);
    fn refresh(&self);
    fn get_style(&self) -> &Style;
}

pub struct Button1 {
    style: Style,
    font: Rc<Font>,
    text: String,
    tx: Sender<StateAction>,
}

impl Button1 {
    pub fn new(style: &Style, font: Rc<Font>, count: i32, tx: Sender<StateAction>) -> Box<Button1> {
        let text = format!("{}", count);

        let TextDimensions { width, height, .. } =
            text::measure_text(&text, Some(&font), *style.font_size.borrow() as u16, 1.0);

        Box::new(Button1 {
            text,
            font,
            style: Style {
                x: Value::Absolute(0.0),
                y: Value::Absolute(0.0),
                height: Value::Absolute(height + 20.0),
                width: Value::Absolute(width + 20.0),
                padding_x: Some(Value::Absolute(10.0)),
                padding_y: Some(Value::Absolute(10.0)),
                font_size: Rc::clone(&style.font_size),
                border: Some(BorderParams {
                    size: 2.0,
                    color: Rc::clone(&style.theme.text),
                }),
                theme: Theme {
                    bg: Rc::clone(&style.theme.bg),
                    ghost: Rc::clone(&style.theme.ghost),
                    text: Rc::clone(&style.theme.text),
                    error: Rc::clone(&style.theme.error),
                },
                ..Style::default()
            },
            tx,
        })
    }
}

impl Component for Button1 {
    fn onclick(&self) {
        self.tx
            .send(StateAction::ButtonCounterInc { inc: 2 })
            .unwrap();
    }
    fn onhover(&self) {
        self.style.draw_border();
    }

    fn refresh(&self) {
        app::text::print_text(
            &self.style,
            &self.text,
            PrintOptions {
                font: Some(Rc::clone(&self.font)),
                ..PrintOptions::default()
            },
        );
    }

    fn get_style(&self) -> &Style {
        &self.style
    }
}
