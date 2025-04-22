use macroquad::{input, window};

use self::component::center::Center;
use self::component::{Axis, Component, Input};

use super::{util, App};

pub mod component;

mod style;
pub use style::Style;

pub async fn run(app: &mut App) {
    let mut components: Vec<Box<dyn Component>> = bombparty(&app.style);

    loop {
        window::clear_background(*app.style.theme.bg.borrow());

        let is_mouse_pressed = input::is_mouse_button_pressed(input::MouseButton::Left);

        for component in components.iter_mut() {
            component.refresh();
            component.handle_hover(is_mouse_pressed);
        }

        util::draw_midpoint();

        window::next_frame().await;
    }
}

pub fn bombparty(style: &crate::app::Style) -> Vec<Box<dyn Component>> {
    vec![Box::new(Center {
        style: Style {
            width: window::screen_width(),
            height: window::screen_height(),
            ..Style::from(style)
        },
        axis: Axis::Both,
        child: Box::new(Input::new(Style {
            ..Style::from(style)
        })),
    })]
}
