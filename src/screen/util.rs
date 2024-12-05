use std::cell::RefCell;
use std::rc::Rc;

use macroquad::input;
use macroquad::math::Vec2;

use super::component::Style;

pub fn handle_mouse_focus(style: &Style, id: i32, focus: Rc<RefCell<i32>>) {
    let (x, y) = input::mouse_position();
    let Vec2 { x: dx, y: dy } = input::mouse_delta_position();

    if style.x.get() <= x
        && x <= style.x.get() + style.width.get()
        && style.y.get() <= y
        && y <= style.y.get() + style.height.get()
        && dx != 0.0
        && dy != 0.0
    {
        *focus.borrow_mut() = id;
    }
}

pub fn is_hover(style: &Style) -> bool {
    let (x, y) = input::mouse_position();
    let Vec2 { x: dx, y: dy } = input::mouse_delta_position();

    if style.x.get() <= x
        && x <= style.x.get() + style.width.get()
        && style.y.get() <= y
        && y <= style.y.get() + style.height.get()
        && dx != 0.0
        && dy != 0.0
    {
        return true;
    }

    false
}
