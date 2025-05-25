pub mod style;
use style::bombparty::Style;

pub mod theme;

pub use macros::StyledComponent;

pub trait StyledComponent {
    fn get_style(&self) -> &Style;
    fn get_style_mut(&mut self) -> &mut Style;
}
