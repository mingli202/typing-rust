use crate::app::bombparty::Style;

use super::{Axis, Component, StyledComponent};

#[derive(StyledComponent)]
pub struct Center {
    pub child: Box<dyn Component>,
    pub style: Style,
    pub axis: Axis,
}

impl Center {
    pub fn new(style: Style, axis: Axis, child: Box<dyn Component>) -> Self {
        Center { style, axis, child }
    }
}

impl Component for Center {
    fn build(&mut self) {
        self.child.build();
    }
    fn refresh(&mut self) {
        let child = self.child.get_style_mut();

        match self.axis {
            Axis::X => child.x = self.style.x + (self.style.width - child.width) / 2.0,
            Axis::Y => child.y = self.style.y + (self.style.height - child.height) / 2.0,
            Axis::Both => {
                child.x = self.style.x + (self.style.width - child.width) / 2.0;
                child.y = self.style.y + (self.style.height - child.height) / 2.0;
            }
        };

        self.child.refresh();
    }
    fn handle_hover(&mut self, is_mouse_pressed: bool) {
        self.child.handle_hover(is_mouse_pressed);
    }
}
