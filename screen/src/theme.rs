use macroquad::color::Color;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Theme {
    pub bg: Rc<RefCell<Color>>,
    pub text: Rc<RefCell<Color>>,
    pub error: Rc<RefCell<Color>>,
    pub ghost: Rc<RefCell<Color>>,
}

impl Theme {
    fn new() -> Self {
        Theme {
            bg: Rc::new(RefCell::new(Color::new(0.0, 0.0, 0.0, 1.0))),
            text: Rc::new(RefCell::new(Color::new(1.0, 1.0, 1.0, 1.0))),
            error: Rc::new(RefCell::new(Color::new(1.0, 0.0, 0.0, 1.0))),
            ghost: Rc::new(RefCell::new(Color::new(1.0, 1.0, 1.0, 0.5))),
        }
    }

    fn set(&mut self, bg: u32, text: u32, error: u32, ghost: Option<u32>) {
        *self.bg.borrow_mut() = Color::from_hex(bg);
        *self.text.borrow_mut() = Color::from_hex(text);
        *self.error.borrow_mut() = Color::from_hex(error);
        *self.ghost.borrow_mut() = if let Some(color) = ghost {
            Color::from_hex(color)
        } else {
            Color {
                a: 0.5,
                ..Color::from_hex(text)
            }
        }
    }

    pub fn set_atom(&mut self) {
        self.set(0x161719, 0xc5c8c6, 0xfd5ff1, Some(0x444444))
    }

    pub fn set_gruvbox(&mut self) {
        self.set(0x1b1b1b, 0xebdbb2, 0xcc241d, Some(0x665c54))
    }

    pub fn set_catppuccin(&mut self) {
        self.set(0x1e1e2e, 0xcdd6f4, 0xf38ba8, Some(0x585b70))
    }

    pub fn set_tokyonight(&mut self) {
        self.set(0x1a1b26, 0xc0caf5, 0xf7768e, Some(0x33467c))
    }
}

impl Default for Theme {
    fn default() -> Self {
        let mut theme = Self::new();
        theme.set_gruvbox();
        theme
    }
}
