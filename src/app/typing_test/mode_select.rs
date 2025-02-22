use std::rc::Rc;

use crate::app::style::Style;
use crate::app::theme::Theme;
use crate::app::{self, Mode};

pub struct ModeSelect {
    style: Style,
    all_modes: Vec<&'static str>,
}

impl ModeSelect {
    pub fn new(style: &Style) -> Self {
        ModeSelect {
            style: Style {
                theme: Theme {
                    bg: Rc::clone(&style.theme.bg),
                    text: Rc::clone(&style.theme.ghost),
                    ghost: Rc::clone(&style.theme.ghost),
                    error: Rc::clone(&style.theme.error),
                },
                font_size: Rc::clone(&style.font_size),
                ..Style::default()
            },
            all_modes: vec!["Words 10", "Words 30", "Words 50", "Words 100", "Quote"],
        }
    }

    pub fn update(&self, mode: &Mode, is_hover: &bool) {
        let text = mode.to_string();
        app::text::print_text(
            &self.style,
            &text[..],
            self.style.x.get(&self.style),
            self.style.y.get(&self.style),
        );
    }
}
