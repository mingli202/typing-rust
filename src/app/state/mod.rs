use std::cell::RefCell;
use std::rc::Rc;

pub mod screen;
pub mod typing_box;

pub struct State<T, A> {
    pub val: Rc<RefCell<T>>,
    pub reducer: fn(Rc<RefCell<T>>, A),
}

impl<T, A> State<T, A> {
    pub fn new(initial_state: T, reducer: fn(Rc<RefCell<T>>, A)) -> Self {
        State {
            val: Rc::new(RefCell::new(initial_state)),
            reducer,
        }
    }

    pub fn dispatch(&self, action: A) {
        (self.reducer)(Rc::clone(&self.val), action);
    }
}

//impl State {
//    pub fn new(data: Data, config: Config) -> Self {
//        State {
//            mode: Rc::new(RefCell::new(Mode::from_quote(
//                data.get_random_quote().clone(),
//            ))),
//            wpm: Rc::new(RefCell::new(0)),
//            style: Style {
//                font_size: Rc::new(RefCell::new(config.font_size)),
//                theme: Theme::get_theme(&config.theme),
//                ..Style::default()
//            },
//            config,
//            data,
//            screen: Rc::new(RefCell::new(app::TypingTest)),
//            typingtest: TypingtestState::default(),
//        }
//    }
//
//    pub fn dispatch(&self, action: Action) {
//        match action {
//            Action::FontChange(n) => *self.style.font_size.borrow_mut() += n,
//            Action::ScreenChange(scr) => *self.screen.borrow_mut() = scr,
//            Action::WpmChange(f) => *self.wpm.borrow_mut() = f,
//
//            // typing test
//            Action::TypingtestClick(typingbox) => {
//                input::clear_input_queue();
//                match *self.typingtest.focus.borrow() {
//                    NextButton => {
//                        self.mode.borrow_mut().next(&self.data);
//                        typingbox.refresh(self.mode.borrow().get_text());
//                        *self.typingtest.focus.borrow_mut() = Nothing;
//                    }
//                    RestartButton => {
//                        typingbox.refresh(self.mode.borrow().get_text());
//                        *self.typingtest.focus.borrow_mut() = Nothing;
//                    }
//                    ThemeButton => {
//                        *self.screen.borrow_mut() = app::ThemeSelect;
//                    }
//                    _ => (),
//                }
//            }
//            Action::TypingTestFocusChange(focus) => *self.typingtest.focus.borrow_mut() = focus,
//        }
//    }
//}
//
//pub enum Action<'a> {
//    FontChange(f32),
//    ScreenChange(Screen),
//    WpmChange(u16),
//
//    TypingtestClick(&'a mut TextBox),
//    TypingTestFocusChange(TypingTestFocus),
//}
//
//#[derive(Default)]
//pub struct TypingtestState {
//    pub focus: Rc<RefCell<TypingTestFocus>>,
//}
//
//pub enum TypingtestAction {
//    Click,
//}
