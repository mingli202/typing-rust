use std::rc::Rc;

use crate::app::style::Style;
use crate::app::theme::Theme;

pub struct Graph {
    incremental_wpm: Vec<u16>,
    style: Style,
}

impl Graph {
    pub fn new(style: &Style, incremental_wpm: Vec<u16>) -> Self {
        Graph {
            incremental_wpm,
            style: Style {
                theme: Theme {
                    text: Rc::clone(&style.theme.text),
                    ghost: Rc::clone(&style.theme.ghost),
                    error: Rc::clone(&style.theme.error),
                    bg: Rc::clone(&style.theme.bg),
                },
                font_size: Rc::clone(&style.font_size),
                ..Style::default()
            },
        }
    }
}
