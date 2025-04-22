use std::cell::RefCell;
use std::rc::Rc;

use macroquad::color::Color;
use macroquad::shapes;

use crate::app::bombparty::Style;

use super::Component;

pub struct Container {
    pub child: Box<dyn Component>,
    pub style: Style,
    pub padding: Padding,
    pub border: Option<Border>,
}

impl Container {
    pub fn new(style: Style, child: Box<dyn Component>) -> Self {
        let mut c = Container {
            child,
            style,
            padding: Padding::new(0.0),
            border: None,
        };
        c.build();

        c
    }
}

impl Component for Container {
    fn build(&mut self) {
        if self.style.width == 0.0 {
            self.style.fit_width = true;
        }
        if self.style.height == 0.0 {
            self.style.fit_height = true;
        }

        self.child.build();
        let child = self.child.get_style_mut();

        if self.style.fit_width {
            self.style.width = child.width + self.padding.l + self.padding.r;
        }
        if self.style.fit_height {
            self.style.height = child.height + self.padding.t + self.padding.b;
        }
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

        if let Some(border) = &self.border {
            let Style {
                x,
                y,
                width,
                height,
                ..
            } = &self.style;

            shapes::draw_line(
                x - border.l / 2.0,
                *y - border.t,
                x - border.l / 2.0,
                y + height + border.b,
                border.l,
                *border.color.borrow(),
            );
            shapes::draw_line(
                x + width + border.r / 2.0,
                *y - border.t,
                x + width + border.r / 2.0,
                y + height + border.b,
                border.r,
                *border.color.borrow(),
            );
            shapes::draw_line(
                *x - border.l,
                y - border.t / 2.0,
                x + width + border.r,
                y - border.t / 2.0,
                border.t,
                *border.color.borrow(),
            );
            shapes::draw_line(
                *x - border.l,
                y + height + border.b / 2.0,
                x + width + border.r,
                y + height + border.b / 2.0,
                border.b,
                *border.color.borrow(),
            );
        }

        self.child.refresh();
    }

    fn handle_hover(&mut self, is_mouse_pressed: bool) {
        self.child.handle_hover(is_mouse_pressed);
    }
}

#[derive(Default, Clone, Debug)]
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

#[derive(Default, Clone, Debug)]
pub struct Border {
    pub l: f32,
    pub r: f32,
    pub t: f32,
    pub b: f32,
    pub color: Rc<RefCell<Color>>,
    pub style: BorderStyle,
}

impl Border {
    pub fn new(border_thickness: f32, color: Rc<RefCell<Color>>) -> Border {
        Border {
            l: border_thickness,
            r: border_thickness,
            t: border_thickness,
            b: border_thickness,
            color,
            ..Border::default()
        }
    }
    pub fn x(self, border_thickness: f32) -> Border {
        Border {
            l: border_thickness,
            r: border_thickness,
            ..self
        }
    }
    pub fn y(self, border_thickness: f32) -> Border {
        Border {
            t: border_thickness,
            b: border_thickness,
            ..self
        }
    }
    pub fn l(self, border_thickness: f32) -> Border {
        Border {
            l: border_thickness,
            ..self
        }
    }
    pub fn r(self, border_thickness: f32) -> Border {
        Border {
            r: border_thickness,
            ..self
        }
    }
    pub fn t(self, border_thickness: f32) -> Border {
        Border {
            t: border_thickness,
            ..self
        }
    }
    pub fn b(self, border_thickness: f32) -> Border {
        Border {
            b: border_thickness,
            ..self
        }
    }
    pub fn style(self, border_style: BorderStyle) -> Border {
        Border {
            style: border_style,
            ..self
        }
    }
    pub fn color(self, border_color: Rc<RefCell<Color>>) -> Border {
        Border {
            color: border_color,
            ..self
        }
    }
}

#[derive(Default, Debug, Clone)]
pub enum BorderStyle {
    #[default]
    Solid,
    Rounded,
}
