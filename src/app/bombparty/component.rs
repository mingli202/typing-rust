use crate::app::Style;

pub trait Component {
    fn onclick(&mut self) {}
    fn onhover(&mut self) {}
    fn refresh(&mut self);
    fn get_style(&self) -> &Style;
}
