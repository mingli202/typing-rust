use crate::app::bombparty::style::Style;

use super::Component;

pub struct Center {
    pub child: Box<dyn Component>,
    pub style: Style,
}

impl Component for Center {
    fn get_style(&self) -> &Style {
        &self.style
    }
    fn get_style_mut(&mut self) -> &mut Style {
        &mut self.style
    }
    fn refresh(&mut self) {
        let child = self.child.get_style_mut();
        child.x = self.style.x + (self.style.width - child.width) / 2.0;
        child.y = self.style.y + (self.style.height - child.height) / 2.0;

        self.child.refresh();
    }
    fn while_hover(&mut self, is_mouse_pressed: bool) {
        self.child.while_hover(is_mouse_pressed);
    }
}
