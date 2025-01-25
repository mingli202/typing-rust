use std::cmp::Ordering;
use std::rc::Rc;

use macroquad::window;

use crate::app;
use crate::app::state::textbox::{reducer, TypingAction, TypingState};
use crate::app::state::State;
use crate::app::{theme::Theme, BorderParams, Style, Value};

pub struct TextBox {
    pub style: Style,
    pub state: State<TypingState, TypingAction>,
}

impl TextBox {
    pub fn new(style: &Style, text: &str) -> TextBox {
        let f1 = Rc::clone(&style.font_size);
        let f2 = Rc::clone(&style.font_size);

        TextBox {
            style: Style {
                font_size: Rc::clone(&style.font_size),
                border: Some(BorderParams {
                    size: 2.0,
                    color: Rc::clone(&style.theme.ghost),
                }),
                theme: Theme {
                    bg: Rc::clone(&style.theme.bg),
                    ghost: Rc::clone(&style.theme.ghost),
                    text: Rc::clone(&style.theme.text),
                    error: Rc::clone(&style.theme.error),
                },
                x: Value::Relative(Box::new(|| (0.5 * window::screen_width()) / 2.0)),
                y: Value::Relative(Box::new(move || {
                    (window::screen_height() - *f1.borrow() * 3.0) / 2.0
                })),
                width: Value::Relative(Box::new(|| window::screen_width() / 2.0)),
                height: Value::Relative(Box::new(move || *f2.borrow() * 3.0)),
                clip: true,
                offset_y: None,
                offset_x: None,
                padding_x: None,
                padding_y: None,
            },
            state: State::new(
                TypingState::new(text, Rc::clone(&style.theme.ghost)),
                reducer,
            ),
        }
    }

    pub fn on_type(&self, c: char) -> bool {
        let state = self.state.get();
        if *state.index.borrow() == state.letters.borrow().len() - 1 {
            return true;
        }

        self.state.dispatch(TypingAction::TypeChar(
            c,
            Rc::clone(&self.style.theme.text),
            Rc::clone(&self.style.theme.error),
        ));

        false
    }

    pub fn delete_char(&self) {
        if *self.state.get().index.borrow() == 0 {
            return;
        }

        self.state
            .dispatch(TypingAction::DeleteChar(Rc::clone(&self.style.theme.ghost)));
    }

    fn update_position(&self, line_breaks: &[usize]) {
        let mut left: i32 = 0;
        let mut right: i32 = line_breaks.len() as i32 - 1;

        let state = self.state.get();
        let index = *state.index.borrow();

        while left < right {
            let mid = (left + right) / 2;

            match index.cmp(&line_breaks[mid as usize]) {
                Ordering::Less => right = mid - 1,
                Ordering::Greater => left = mid + 1,
                Ordering::Equal => {
                    left = mid;
                    break;
                }
            }
        }

        if left > 0 && index < line_breaks[left as usize] {
            left -= 1;
        }

        let font_size = *self.style.font_size.borrow();
        let scroll = -(left as f32 * font_size);

        if scroll != *state.scroll.borrow() {
            self.state.dispatch(TypingAction::Scroll(scroll));
        }
    }

    pub fn get_wpm(&self, end: Option<usize>) -> u16 {
        // thread::sleep(Duration::from_secs(1));
        let state = self.state.get();
        let end = end.unwrap_or(state.letters.borrow().len());

        let time_passed: u128 = state.time_started.borrow().elapsed().as_millis();

        let mut wrongs = 0.0;
        let mut is_word_wrong = false;

        for i in 0..end {
            let letter = &state.letters.borrow()[i];

            if *letter.color.borrow() == *self.style.theme.error.borrow() && !is_word_wrong {
                wrongs += 1.0;
                is_word_wrong = true;
            }
            if letter.letter == ' ' {
                is_word_wrong = false;
            }
        }

        (1000 * 60 * (end as f32 / 5.0 - wrongs) as u128 / time_passed) as u16
    }

    pub fn get_incremental_wpm(&self) {
        let state = self.state.get();
        let t = state.timer.borrow().elapsed();

        if !*state.started.borrow() || t.as_millis() < 500 {
            return;
        }

        self.state.dispatch(TypingAction::AddWmp(
            self.get_wpm(Some(*state.index.borrow())),
        ));
    }

    pub fn update(&self) {
        self.style.draw_bg();

        let state = self.state.get();

        let line_breaks = app::text::print_letters_wrap(
            &self.style,
            &state.letters.borrow(),
            *state.index.borrow(),
            *state.scroll.borrow(),
        );
        self.update_position(&line_breaks);

        self.style.draw_mask();

        if *state.index.borrow() > 0 && !*state.started.borrow() {
            self.state.dispatch(TypingAction::TimerStart);
        }
        self.get_incremental_wpm();
    }
}
