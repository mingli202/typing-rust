use crate::app::bombparty::Style;

use super::Component;

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
    ) -> Self {
        FlexBox {
            style,
            flex_axis,
            gap,
            children,
            children_dimensions: vec![],
        }
    }
}

impl Component for FlexBox {
    fn get_style(&self) -> &Style {
        &self.style
    }

    fn get_style_mut(&mut self) -> &mut Style {
        &mut self.style
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
        for (i, child) in self.children.iter_mut().enumerate() {
            child.get_style_mut().x = self.children_dimensions[i].0 + self.style.x;
            child.get_style_mut().y = self.children_dimensions[i].1 + self.style.y;

            child.refresh();
        }
    }
}

pub enum FlexAxis {
    X,
    Y,
}
