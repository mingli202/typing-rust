use std::rc::Rc;

use macroquad::{input, window};
use tokio::time::Instant;

use self::component::flexbox::FlexAxis;
use self::component::{Axis, Border, Center, Component, Container, FlexBox, Input, Padding, Text};

use super::App;

pub mod component;
pub mod schemas;

pub mod style;
use style::Style;

pub async fn run(app: &mut App) {
    let app_style = Style::from(&app.style);

    let mut root = Container::new(
        app_style.clone(),
        FlexBox::new(
            Style {
                x: 10.0,
                y: 10.0,
                ..app_style.clone()
            },
            FlexAxis::X,
            20.0,
            vec![
                Box::new(Container {
                    style: app_style.clone(),
                    padding: Padding::new(10.0),
                    border: Some(Border::new(2.0, Rc::clone(&app_style.theme.text))),
                    child: Text::new(app_style.clone(), "Hello world".to_string()),
                }),
                Input::new(app_style.clone(), None),
            ],
        ),
    );
    root.border = Some(Border::new(2.0, Rc::clone(&app_style.theme.text)));
    root.padding = Padding::new(5.0);

    let mut root = Center::new(app_style.clone(), Axis::Both, root);
    root.style.width = window::screen_width();
    root.style.height = window::screen_height();

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
            root.style.width = w;
            root.style.height = h;
            root.build();
        }
    }
}
