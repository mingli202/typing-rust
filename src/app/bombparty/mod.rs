use std::rc::Rc;
use std::sync::mpsc;

use macroquad::window;

use self::component::{Button1, Component};

use super::{util, App};

mod component;
mod macros;
mod state;

pub async fn run(app: &mut App) {
    let mut state = State::default();
    let (tx, rx) = mpsc::channel::<StateAction>();

    let mut components: Vec<Box<dyn Component>> = vec![Button1::new(
        &app.style,
        Rc::clone(&app.font),
        state.counter,
        tx.clone(),
    )];

    loop {
        window::clear_background(*app.style.theme.bg.borrow());

        for component in components.iter_mut() {
            component.as_mut().refresh();

            if util::is_hover(component.get_style()) {
                component.onhover();
            }
        }

        window::next_frame().await;
    }
}

#[derive(Debug, Default)]
struct State {
    counter: i32,
}

enum StateAction {
    ButtonCounterInc { inc: i32 },
}
