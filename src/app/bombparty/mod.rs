use macroquad::{input, window};
use tokio::time::Instant;

use self::component::center::Center;
use self::component::{Axis, Component, Input};

use super::App;

pub mod component;

mod style;
pub use style::Style;

pub async fn run(app: &mut App) {
    let app_style = Style::from(&app.style);

    let mut root = Center {
        style: Style {
            width: window::screen_width(),
            height: window::screen_height(),
            ..app_style.clone()
        },
        axis: Axis::Both,
        child: Box::new(Input::new(Style {
            width: 500.0,
            ..app_style
        })),
    };
    root.build();

    let mut w = root.style.width;
    let mut h = root.style.height;
    let mut timer = Instant::now();
    let mut did_resize = false;

    loop {
        window::clear_background(*app.style.theme.bg.borrow());

        let is_mouse_pressed = input::is_mouse_button_pressed(input::MouseButton::Left);

        root.refresh();
        root.handle_hover(is_mouse_pressed);

        // util::draw_midpoint();

        window::next_frame().await;

        let scr_w = window::screen_width();
        let scr_h = window::screen_height();

        if (w - scr_w).abs() > 0.01 || (h - scr_h).abs() > 0.01 {
            timer = Instant::now();
            w = scr_w;
            h = scr_h;
            did_resize = true;
        } else if did_resize && timer.elapsed().as_millis() > 100 {
            did_resize = false;
            root.style.width = w;
            root.style.height = h;
            root.build();
        }
    }
}
