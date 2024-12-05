use macroquad::input;

use super::component::Style;

pub fn is_hover(style: &Style) -> bool {
    let (x, y) = input::mouse_position();

    if style.x.get() <= x
        && x <= style.x.get() + style.width.get()
        && style.y.get() <= y
        && y <= style.y.get() + style.height.get()
    {
        return true;
    }

    false
}
