/// Whether the layout fits the content
/// grows to take all available space
/// or is of a fixed size
#[derive(Default, Debug, Clone, PartialEq, PartialOrd)]
pub enum SizingType {
    #[default]
    Fit,
    Grow,
    Fixed(i32),
}

/// Determines the width and height of the layout
#[derive(Default, Debug, Clone, PartialEq, PartialOrd)]
pub struct Sizing {
    width: SizingType,
    height: SizingType,
}

impl Sizing {
    pub fn new(width: SizingType, height: SizingType) -> Self {
        Sizing { width, height }
    }
}

/// Whether to oriente the children vertially or horizontally
#[derive(Default, Debug, Clone, PartialEq, PartialOrd)]
pub enum Direction {
    #[default]
    Vertical,
    Horizontal,
}

/// How to align the children of this layout
#[derive(Default, Debug, Clone, PartialEq, PartialOrd)]
pub enum Alignment {
    #[default]
    Start,
    End,
    Between,
    Center,
}

#[derive(Default, Debug, Clone, PartialEq, PartialOrd)]
pub struct Flex {
    direction: Direction,
    alignment: Alignment,
    gap: i32,
}

impl Flex {
    pub fn new(direction: Direction, alignment: Alignment, gap: i32) -> Self {
        Flex {
            direction,
            alignment,
            gap,
        }
    }

    pub fn with_direction(self, direction: Direction) -> Self {
        Flex::new(direction, self.alignment, self.gap)
    }
    pub fn with_alignment(self, alignment: Alignment) -> Self {
        Flex::new(self.direction, alignment, self.gap)
    }
    pub fn with_gap(self, gap: i32) -> Self {
        Flex::new(self.direction, self.alignment, gap)
    }
}

/// The bounding rectangle relative to its parent
#[derive(Default, Debug, Clone, PartialEq, PartialOrd)]
pub struct BoundingBox {
    top: i32,
    left: i32,
    width: i32,
    height: i32,
}

#[derive(Default, Debug, Clone, PartialEq, PartialOrd)]
pub struct ComputedLayout {
    bounding_box: BoundingBox,
    children: Vec<ComputedLayout>,
}

impl ComputedLayout {
    pub fn new(bounding_box: BoundingBox, children: Vec<ComputedLayout>) -> Self {
        ComputedLayout {
            bounding_box,
            children,
        }
    }

    pub fn with_children(self, children: Vec<ComputedLayout>) -> Self {
        ComputedLayout::new(self.bounding_box, children)
    }

    pub fn get_max_width_height(&self) -> (i32, i32) {
        let ((min_x, max_x), (min_y, max_y)) = self.get_min_max_x_y();

        (i32::abs(max_x - min_x), i32::abs(max_y - min_y))
    }

    fn get_min_max_x_y(&self) -> ((i32, i32), (i32, i32)) {
        let BoundingBox {
            left,
            top,
            width,
            height,
        } = self.bounding_box;

        let (min_x, max_x, min_y, max_y) = (left, left + width, top, top + height);

        let ((child_min_x, child_max_x), (child_min_y, child_max_y)) =
            ComputedLayout::get_children_min_max_x_y(&self.children);

        (
            (i32::min(child_min_x, min_x), i32::max(child_max_x, max_x)),
            (i32::min(child_min_y, min_y), i32::max(child_max_y, max_y)),
        )
    }

    pub fn get_children_max_width_height(children: &[ComputedLayout]) -> (i32, i32) {
        let ((min_x, max_x), (min_y, max_y)) = ComputedLayout::get_children_min_max_x_y(children);

        (i32::abs(max_x - min_x), i32::abs(max_y - min_y))
    }

    fn get_children_min_max_x_y(children: &[ComputedLayout]) -> ((i32, i32), (i32, i32)) {
        let mut first = children.iter().take(1);
        let rest = children.iter().skip(1);

        match first.next() {
            Some(child) => {
                let BoundingBox {
                    top,
                    left,
                    width,
                    height,
                } = child.bounding_box;

                rest.fold(((left, left + width), (top, top + height)), |acc, child| {
                    let ((child_min_x, child_max_x), (child_min_y, child_max_y)) =
                        child.get_min_max_x_y();

                    let ((min_x, max_x), (min_y, max_y)) = acc;

                    (
                        (i32::min(child_min_x, min_x), i32::max(child_max_x, max_x)),
                        (i32::min(child_min_y, min_y), i32::max(child_max_y, max_y)),
                    )
                })
            }
            None => ((0, 0), (0, 0)),
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, PartialOrd)]
pub struct Layout {
    sizing: Sizing,
    flex: Flex,
    children: Vec<Layout>,
}

impl Layout {
    pub fn new(sizing: Sizing, flex: Flex, children: Vec<Layout>) -> Self {
        Layout {
            sizing,
            flex,
            children,
        }
    }

    pub fn with_sizing(self, sizing: Sizing) -> Self {
        Layout { sizing, ..self }
    }

    pub fn with_flex(self, flex: Flex) -> Self {
        Layout { flex, ..self }
    }

    pub fn with_children(self, children: Vec<Layout>) -> Self {
        Layout { children, ..self }
    }

    /// Computes a renderable layout box for gui libraries to use
    pub fn compute(self) -> ComputedLayout {
        let sizing = &self.sizing;

        let width = match sizing.width {
            SizingType::Fixed(val) => val,
            _ => panic!("Root layout must have a fixed width"),
        };

        let height = match sizing.height {
            SizingType::Fixed(val) => val,
            _ => panic!("Root layout must have a fixed height"),
        };

        let bounding_box = BoundingBox {
            top: 0,
            left: 0,
            width,
            height,
        };

        ComputedLayout {
            children: self
                .children
                .into_iter()
                .map(|child| child.compute_child(&bounding_box))
                .collect(),
            bounding_box,
        }
    }

    fn compute_child(self, parent_bounding_box: &BoundingBox) -> ComputedLayout {
        let mut bounding_box = BoundingBox {
            top: 0,
            left: 0,
            width: match self.sizing.width {
                SizingType::Fit => -1,
                SizingType::Grow => parent_bounding_box.width,
                SizingType::Fixed(w) => w,
            },
            height: match self.sizing.height {
                SizingType::Fit => -1,
                SizingType::Grow => parent_bounding_box.height,
                SizingType::Fixed(w) => w,
            },
        };

        let children: Vec<ComputedLayout> = self
            .children
            .into_iter()
            .map(|child| child.compute_child(&bounding_box))
            .collect();

        if matches!(self.sizing.width, SizingType::Fit)
            || matches!(self.sizing.height, SizingType::Fit)
        {
            let (child_max_width, child_max_height) =
                ComputedLayout::get_children_max_width_height(&children[..]);

            if matches!(self.sizing.width, SizingType::Fit) {
                bounding_box.width = child_max_width;
            }

            if matches!(self.sizing.height, SizingType::Fit) {
                bounding_box.height = child_max_height;
            }
        }

        let Flex {
            direction,
            alignment,
            gap,
        } = self.flex;

        match direction {
            Direction::Vertical => {}
            Direction::Horizontal => {}
        }

        ComputedLayout::default()
    }

    fn compute_flex(self) -> Layout {
        let Flex {
            direction,
            alignment,
            gap,
        } = self.flex;

        match alignment {
            Alignment::Start => {}
            Alignment::End => {}
            Alignment::Center => {}
            Alignment::Between => {}
        }

        Layout::default()
    }

    fn align(self, x: i32, y: i32) -> Layout {
        Layout::default()
    }

    /// Computes the final size of this layout
    fn compute_size(self, parent_width: i32, parent_height: i32) -> ComputedLayout {
        let children: Vec<ComputedLayout> = self
            .children
            .into_iter()
            .map(|child| child.compute_size(parent_width, parent_height))
            .collect();

        let width = self.sizing.width;
        let height = self.sizing.height;

        let (children_max_width, children_max_height) =
            ComputedLayout::get_children_max_width_height(&children);

        let computed_layout = ComputedLayout::default().with_children(children);

        let width = match width {
            SizingType::Fit => children_max_width,
            SizingType::Grow => parent_width,
            SizingType::Fixed(w) => w,
        };

        let height = match height {
            SizingType::Fit => children_max_height,
            SizingType::Grow => parent_height,
            SizingType::Fixed(w) => w,
        };

        ComputedLayout {
            bounding_box: BoundingBox {
                top: 0,
                left: 0,
                width,
                height,
            },
            children: computed_layout.children,
        }
    }
}

#[cfg(test)]
mod layout_test {
    use super::*;

    #[test]
    fn flex_start() {
        let layout = Layout::default()
            .with_sizing(Sizing {
                width: SizingType::Fixed(100),
                height: SizingType::Fixed(100),
            })
            .with_children(vec![
                Layout::default().with_sizing(Sizing {
                    width: SizingType::Fixed(10),
                    height: SizingType::Fixed(10),
                }),
                Layout::default().with_sizing(Sizing {
                    width: SizingType::Fixed(10),
                    height: SizingType::Fixed(30),
                }),
                Layout::default().with_sizing(Sizing {
                    width: SizingType::Fixed(10),
                    height: SizingType::Fixed(4),
                }),
            ]);

        let computed_layout = layout.compute();

        let expected = ComputedLayout {
            bounding_box: BoundingBox {
                top: 0,
                left: 0,
                width: 100,
                height: 100,
            },
            children: vec![
                ComputedLayout {
                    bounding_box: BoundingBox {
                        top: 0,
                        left: 0,
                        width: 10,
                        height: 10,
                    },
                    children: vec![],
                },
                ComputedLayout {
                    bounding_box: BoundingBox {
                        top: 10,
                        left: 0,
                        width: 10,
                        height: 30,
                    },
                    children: vec![],
                },
                ComputedLayout {
                    bounding_box: BoundingBox {
                        top: 40,
                        left: 0,
                        width: 10,
                        height: 4,
                    },
                    children: vec![],
                },
            ],
        };

        assert_eq!(computed_layout, expected)
    }
}
