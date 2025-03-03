use std::rc::Rc;

use macroquad::color::Color;
use macroquad::text::{self, Font};

use crate::app::style::Style;
use crate::app::text::PrintOptions;
use crate::app::theme::Theme;
use crate::app::{self, util, BorderParams, Mode, Value};
use crate::data_provider::{Data, Quote};

pub struct ModeSelect {
    style: Style,
    buttons: Vec<Button>,
    word_buttons: Vec<Button>,
    pub next_mode_selected: Option<Mode>,
}

impl ModeSelect {
    pub fn new(style: &Style, font: Rc<Font>) -> Self {
        ModeSelect {
            next_mode_selected: None,
            style: Style {
                y: Value::Absolute(10.0),
                x: Value::Absolute(10.0),
                font_size: Rc::clone(&style.font_size),
                ..Style::default()
            },
            buttons: [
                (
                    "Words",
                    Mode::Words {
                        n: 10,
                        s: "".to_string(),
                    },
                ),
                (
                    "Quote",
                    Mode::Quote(Quote {
                        source: "".to_string(),
                        quote: "".to_string(),
                    }),
                ),
            ]
            .into_iter()
            .map(|(s, m)| Button::new(s.to_string(), m, style, Rc::clone(&font)))
            .collect(),
            word_buttons: [
                (
                    "10",
                    Mode::Words {
                        n: 10,
                        s: "".to_string(),
                    },
                ),
                (
                    "30",
                    Mode::Words {
                        n: 30,
                        s: "".to_string(),
                    },
                ),
                (
                    "50",
                    Mode::Words {
                        n: 50,
                        s: "".to_string(),
                    },
                ),
                (
                    "100",
                    Mode::Words {
                        n: 100,
                        s: "".to_string(),
                    },
                ),
            ]
            .into_iter()
            .map(|(s, m)| Button::new(s.to_string(), m, style, Rc::clone(&font)))
            .collect(),
        }
    }

    pub fn update(&mut self, mode: &Mode) {
        let text = mode.get_name();

        let mut hover_mode = None;

        let mut x = 0.0;
        for btn in &mut self.buttons {
            let color = if text == btn.text {
                *self.style.theme.text.borrow()
            } else {
                *self.style.theme.ghost.borrow()
            };

            btn.update(x + self.style.x(), self.style.y(), color);

            x += btn.style.width();

            if util::is_hover(&btn.style) {
                btn.style.draw_border();

                hover_mode = Some(btn.mode.clone());
            }
        }

        let y = self.buttons[0].style.height() + self.style.y();

        if let Mode::Words { n, .. } = mode.clone() {
            let mut x = 0.0;
            for btn in &mut self.word_buttons {
                let color = if n == btn.text.parse::<usize>().unwrap() {
                    *self.style.theme.text.borrow()
                } else {
                    *self.style.theme.ghost.borrow()
                };

                btn.update(x + self.style.x(), y, color);

                x += btn.style.width();

                if util::is_hover(&btn.style) {
                    btn.style.draw_border();

                    hover_mode = Some(btn.mode.clone());
                }
            }
        }

        self.next_mode_selected = hover_mode;
    }
}

struct Button {
    text: String,
    style: Style,
    font: Rc<Font>,
    mode: Mode,
}

impl Button {
    pub fn new(text: String, mode: Mode, style: &Style, font: Rc<Font>) -> Self {
        let f1 = Rc::clone(&style.font_size);
        let font1 = Rc::clone(&font);

        let f2 = Rc::clone(&style.font_size);
        let font2 = Rc::clone(&font);
        let t2 = text.clone();

        Button {
            mode,
            text: text.clone(),
            style: Style {
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
                padding_x: Some(Value::Absolute(10.0)),
                padding_y: Some(Value::Absolute(10.0)),
                width: Value::Relative(Box::new(move |this| {
                    2.0 * this.padding_x()
                        + text::measure_text(&text, Some(&font1), *f1.borrow() as u16, 0.7).width
                })),
                height: Value::Relative(Box::new(move |this| {
                    2.0 * this.padding_y()
                        + text::measure_text(&t2, Some(&font2), *f2.borrow() as u16, 0.7).height
                })),
                font_size: Rc::clone(&style.font_size),
                ..Style::default()
            },
            font,
        }
    }

    pub fn update(&mut self, x: f32, y: f32, color: Color) {
        self.style.x = Value::Absolute(x);
        self.style.y = Value::Absolute(y);

        app::text::print_text(
            &self.style,
            &self.text,
            PrintOptions {
                font: Some(Rc::clone(&self.font)),
                font_scale: Some(0.7),
                color: Some(color),
                ..PrintOptions::default()
            },
        );
    }
}
