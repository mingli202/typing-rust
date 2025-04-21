use macroquad::{input, window};

use self::component::Component;

use super::{util, App};

mod component;

pub async fn run(app: &mut App) {
    let mut components: Vec<Box<dyn Component>> = vec![];

    loop {
        window::clear_background(*app.style.theme.bg.borrow());

        let is_mouse_pressed = input::is_mouse_button_pressed(input::MouseButton::Left);

        for component in components.iter_mut() {
            component.as_mut().refresh();

            if util::is_hover(component.get_style()) {
                component.onhover();

                if is_mouse_pressed {
                    component.onclick();
                }
            }
        }

        window::next_frame().await;
    }
}
