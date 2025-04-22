use crate::app::bombparty::Style;

use super::Component;

pub struct Container {
    pub child: Box<dyn Component>,
    pub style: Style,
    pub padding: Padding,
}

impl Container {
    pub fn new(style: Style, child: Box<dyn Component>) -> Self {
        let mut c = Container {
            child,
            style,
            padding: Padding::new(0.0),
        };
        c.build();

        c
    }
}

impl Component for Container {
    fn build(&mut self) {
        self.child.build();
        let child = self.child.get_style_mut();

        self.style.width = child.width + self.padding.l + self.padding.r;
        self.style.height = child.height + self.padding.t + self.padding.b;
    }
    fn get_style(&self) -> &Style {
        &self.style
    }

    fn get_style_mut(&mut self) -> &mut Style {
        &mut self.style
    }

    fn refresh(&mut self) {
        let child = self.child.get_style_mut();

        child.x = self.style.x + self.padding.l;
        child.y = self.style.y + self.padding.t;

        self.child.refresh();
    }

    fn handle_hover(&mut self, is_mouse_pressed: bool) {
        self.child.handle_hover(is_mouse_pressed);
    }
}

#[derive(Default)]
pub struct Padding {
    pub l: f32,
    pub r: f32,
    pub t: f32,
    pub b: f32,
}

impl Padding {
    pub fn x(padding: f32) -> Padding {
        Padding {
            l: padding,
            r: padding,
            ..Padding::default()
        }
    }
    pub fn y(padding: f32) -> Padding {
        Padding {
            t: padding,
            b: padding,
            ..Padding::default()
        }
    }
    pub fn new(padding: f32) -> Padding {
        Padding {
            l: padding,
            r: padding,
            t: padding,
            b: padding,
        }
    }
    pub fn l(padding: f32) -> Padding {
        Padding {
            l: padding,
            ..Padding::default()
        }
    }
    pub fn r(padding: f32) -> Padding {
        Padding {
            r: padding,
            ..Padding::default()
        }
    }
    pub fn t(padding: f32) -> Padding {
        Padding {
            t: padding,
            ..Padding::default()
        }
    }
    pub fn b(padding: f32) -> Padding {
        Padding {
            b: padding,
            ..Padding::default()
        }
    }
}
