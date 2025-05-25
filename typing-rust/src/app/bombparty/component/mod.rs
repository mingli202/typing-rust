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
use types::StyledComponent;

use super::Style;

pub trait Component: StyledComponent {
    fn on_init(&mut self) {}
    fn on_destroy(&mut self) {}
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

pub enum C {
    Center(Center),
    Input(Input),
    FlexBox(FlexBox),
    Text(Text),
    Container(Container),
}

impl StyledComponent for C {
    fn get_style(&self) -> &Style {
        match self {
            C::Text(c) => c.get_style(),
            C::Center(c) => c.get_style(),
            C::Container(c) => c.get_style(),
            C::Input(c) => c.get_style(),
            C::FlexBox(c) => c.get_style(),
        }
    }
    fn get_style_mut(&mut self) -> &mut Style {
        match self {
            C::Text(c) => c.get_style_mut(),
            C::Center(c) => c.get_style_mut(),
            C::Container(c) => c.get_style_mut(),
            C::Input(c) => c.get_style_mut(),
            C::FlexBox(c) => c.get_style_mut(),
        }
    }
}

impl Component for C {
    fn refresh(&mut self) {
        match self {
            C::Text(c) => c.refresh(),
            C::Center(c) => c.refresh(),
            C::Container(c) => c.refresh(),
            C::Input(c) => c.refresh(),
            C::FlexBox(c) => c.refresh(),
        }
    }
}
