pub mod center;
pub mod container;
pub mod flexbox;
pub mod input;
pub mod text;

pub use center::Center;
pub use container::*;
pub use flexbox::FlexBox;
pub use input::Input;
pub use text::Text;

use super::Style;

use typing_rust_macros::StyledComponent;

pub trait StyledComponent {
    fn get_style(&self) -> &Style;
    fn get_style_mut(&mut self) -> &mut Style;
}

pub trait Component: StyledComponent {
    fn on_click_in(&mut self) {}
    fn on_click_out(&mut self) {}

    /// Building means setting the width and height of the Component
    fn build(&mut self) {}

    fn on_hover_in(&mut self) {}
    fn on_hover_out(&mut self) {}
    fn is_hover(&self) -> bool {
        let style = self.get_style();
        let (x, y) = macroquad::input::mouse_position();

        style.x <= x && x <= style.x + style.width && style.y <= y && y <= style.y + style.height
    }
    fn handle_hover(&mut self, is_mouse_pressed: bool) {
        if self.is_hover() {
            self.on_hover_in();

            if is_mouse_pressed {
                self.on_click_in();
            }
        } else {
            self.on_hover_out();

            if is_mouse_pressed {
                self.on_click_out();
            }
        }
    }

    /// Refreshes the x and y positions and render the Component
    fn refresh(&mut self);

    fn boxed(self) -> Box<Self>
    where
        Self: Sized,
    {
        Box::new(self)
    }
}

pub enum Axis {
    Y,
    X,
    Both,
}
