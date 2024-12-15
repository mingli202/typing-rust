use macroquad::color::Color;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::rc::Rc;

pub struct Theme {
    pub bg: Rc<RefCell<Color>>,
    pub text: Rc<RefCell<Color>>,
    pub error: Rc<RefCell<Color>>,
    pub ghost: Rc<RefCell<Color>>,
}

#[derive(PartialEq, Hash, Eq, Debug, Clone, Serialize, Deserialize, Default)]
pub enum ThemeName {
    Catppuccin,
    Atom,
    Tokyonight,

    #[default]
    Gruvbox,
}

impl Theme {
    pub fn new() -> Self {
        Theme {
            bg: Rc::new(RefCell::new(Color::new(0.0, 0.0, 0.0, 1.0))),
            text: Rc::new(RefCell::new(Color::new(1.0, 1.0, 1.0, 1.0))),
            error: Rc::new(RefCell::new(Color::new(1.0, 0.0, 0.0, 1.0))),
            ghost: Rc::new(RefCell::new(Color::new(1.0, 1.0, 1.0, 0.5))),
        }
    }

    pub fn set(&self, theme_name: &ThemeName) {
        let (bg, text, error, ghost) = match theme_name {
            ThemeName::Atom => (0x161719, 0xc5c8c6, 0xfd5ff1, 0x444444),
            ThemeName::Gruvbox => (0x1b1b1b, 0xebdbb2, 0xcc241d, 0x665c54),
            ThemeName::Catppuccin => (0x1e1e2e, 0xcdd6f4, 0xf38ba8, 0x585b70),
            ThemeName::Tokyonight => (0x1a1b26, 0xc0caf5, 0xf7768e, 0x33467c),
        };

        *self.bg.borrow_mut() = Color::from_hex(bg);
        *self.text.borrow_mut() = Color::from_hex(text);
        *self.error.borrow_mut() = Color::from_hex(error);
        *self.ghost.borrow_mut() = Color::from_hex(ghost);
    }

    pub fn get_theme(theme_name: &ThemeName) -> Self {
        let theme = Self::new();
        theme.set(theme_name);
        theme
    }
}

impl Default for Theme {
    fn default() -> Self {
        let theme = Self::new();
        theme.set(&ThemeName::default());
        theme
    }
}
