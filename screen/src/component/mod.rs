mod textbox;
pub use textbox::TextBox;

pub trait Component {
    /// Function that will be called on each frame
    fn update(&self);
}

enum Value {
    Relative(Box<dyn Fn(f32) -> f32>),
    Absolute(f32),
}

struct Shape {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}
