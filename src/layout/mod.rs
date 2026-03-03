/// Whether the layout fits the content
/// grows to take all available space
/// or is of a fixed size
#[derive(Default)]
pub enum SizingType {
    #[default]
    Fit,
    Grow(f64),
    Fixed(i32),
}

#[derive(Default)]
pub struct Sizing {
    width: SizingType,
    height: SizingType,
}

#[derive(Default)]
pub enum FlexDirection {
    #[default]
    Vertical,
    Horizontal,
}

/// The bounding rectangle relative to its parent
#[derive(Default)]
pub struct BoundingRect {
    top: i32,
    left: i32,
    width: i32,
    height: i32,
}

#[derive(Default)]
pub struct Layout {
    bounding_rect: BoundingRect,
    sizing: Sizing,
    flex: FlexDirection,
    children: Vec<Layout>,
}
