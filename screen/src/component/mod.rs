mod textbox;

pub use textbox::TextBox;

pub trait Component {
    /// Function that will be called on each frame
    fn update(&self);
}

pub enum Value<T> {
    Relative(Box<dyn Fn() -> T>),
    Absolute(T),
}

impl<T: Clone> Value<T> {
    fn get(&self) -> T {
        match self {
            Self::Absolute(v) => v.clone(),
            Self::Relative(v) => v(),
        }
    }
}

struct Shape {
    x: Value<f32>,
    y: Value<f32>,
    width: Value<f32>,
    height: Value<f32>,
}
