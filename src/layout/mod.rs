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
            bounding_box,
            children: self
                .children
                .into_iter()
                .map(move |child| child.compute_child(&bounding_box))
                .collect(),
        }
    }

    fn compute_child(self, parent_bounding_box: &BoundingBox) -> ComputedLayout {
        ComputedLayout::default()
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
