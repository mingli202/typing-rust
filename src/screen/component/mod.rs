pub mod next_button;
pub mod quit_button;
pub mod restart_button;
pub mod textbox;
pub mod theme_button;
pub mod tracker;
pub mod wpm;

pub mod style;
pub use style::{BorderParams, Style};

pub trait Component {
    /// Function that will be called on each frame
    fn get_style(&self) -> Option<&Style> {
        None
    }
}

pub enum Value<T> {
    Relative(Box<dyn Fn() -> T>),
    Absolute(T),
}

impl<T: Clone> Value<T> {
    pub fn get(&self) -> T {
        match self {
            Self::Absolute(v) => v.clone(),
            Self::Relative(v) => v(),
        }
    }
}
