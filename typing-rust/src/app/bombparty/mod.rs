use types::StyledComponent;

use macroquad::{input, window};
use tokio::time::Instant;

use self::component::flexbox::FlexAxis;
use self::component::{Axis, Center, Component, FlexBox, Text, C};

use super::App;

pub mod component;
pub mod schemas;

pub mod style;
use style::Style;

pub async fn run(app: &mut App) {
    let app_style = Style::from(&app.style);

    let mut root = menu(app_style);

    let mut w = root.get_style().width;
    let mut h = root.get_style().height;
    let mut timer = Instant::now();
    let mut did_resize = false;

    loop {
        window::clear_background(*app.style.theme.bg.borrow());

        let is_mouse_pressed = input::is_mouse_button_pressed(input::MouseButton::Left);

        root.refresh();
        root.handle_hover(is_mouse_pressed);

        super::util::draw_midpoint();

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
            root.get_style_mut().width = w;
            root.get_style_mut().height = h;
            root.build();
        }
    }
}

fn menu(style: Style) -> C {
    let mut root = Center::new(
        style.clone(),
        Axis::Both,
        FlexBox::new(
            style.clone(),
            FlexAxis::Y,
            10.0,
            vec![
                Text::new(style.clone(), "Hello world".to_string()).boxed(),
                Text::new(style.clone(), "Hello world again".to_string()).boxed(),
            ],
        )
        .boxed(),
    );

    root.style.width = window::screen_width();
    root.style.height = window::screen_height();
    root.style.x = 0.0;
    root.style.y = 0.0;

    root.build();

    C::Center(root)
}
