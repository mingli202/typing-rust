use crate::app::bombparty::Style;
use crate::app::util::F32Eq;

use super::{Component, StyledComponent};

#[derive(StyledComponent)]
pub struct FlexBox {
    pub style: Style,
    pub flex_axis: FlexAxis,
    pub gap: f32,
    pub children: Vec<Box<dyn Component>>,
    children_dimensions: Vec<(f32, f32)>,
}

impl FlexBox {
    pub fn new(
        style: Style,
        flex_axis: FlexAxis,
        gap: f32,
        children: Vec<Box<dyn Component>>,
    ) -> Box<Self> {
        Box::new(FlexBox {
            style,
            flex_axis,
            gap,
            children,
            children_dimensions: vec![],
        })
    }
}

impl Component for FlexBox {
    fn handle_hover(&mut self, is_mouse_pressed: bool) {
        for child in self.children.iter_mut() {
            child.handle_hover(is_mouse_pressed);
        }
    }

    fn build(&mut self) {
        if self.style.width == 0.0 {
            self.style.fit_width = true;
        }
        if self.style.height == 0.0 {
            self.style.fit_height = true;
        }

        let mut width = 0.0;
        let mut height = 0.0;

        match self.flex_axis {
            FlexAxis::X => {
                let mut x = 0.0;

                for child in self.children.iter_mut() {
                    self.children_dimensions.push((x, 0.0));

                    child.build();

                    let Style {
                        height: h,
                        width: w,
                        ..
                    } = child.get_style();
                    x += w + self.gap;

                    width += w;

                    if *h > height {
                        height = *h;
                    }
                }

                width += self.gap * (0.max(self.children.len() - 1) as f32);
            }
            FlexAxis::Y => {
                let mut y = 0.0;

                for child in self.children.iter_mut() {
                    self.children_dimensions.push((0.0, y));

                    child.build();

                    let Style {
                        height: h,
                        width: w,
                        ..
                    } = child.get_style();
                    y += h + self.gap;

                    height += h;

                    if *w > width {
                        width = *w;
                    }
                }

                height += self.gap * (0.max(self.children.len() - 1) as f32);
            }
        }

        if self.style.fit_width {
            self.style.width = width;
        }
        if self.style.fit_height {
            self.style.height = height;
        }
    }

    fn refresh(&mut self) {
        let mut width = 0.0;
        let mut height = 0.0;

        for (i, child) in self.children.iter_mut().enumerate() {
            let style_child = child.get_style_mut();

            style_child.x = self.children_dimensions[i].0 + self.style.x;
            style_child.y = self.children_dimensions[i].1 + self.style.y;

            match self.flex_axis {
                FlexAxis::X => {
                    if style_child.height > height {
                        height = style_child.height;
                    }

                    width += style_child.width;

                    if i != 0 {
                        width += self.gap;
                    }
                }
                FlexAxis::Y => {
                    if style_child.width > width {
                        width = style_child.width;
                    }

                    height += style_child.height;

                    if i != 0 {
                        height += self.gap;
                    }
                }
            }

            child.refresh();
        }

        if !self.style.width.eq_approx(&width) || !self.style.height.eq_approx(&height) {
            self.build();
        }
    }
}

pub enum FlexAxis {
    X,
    Y,
}
