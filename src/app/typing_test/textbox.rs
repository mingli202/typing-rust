use std::cmp::Ordering;
use std::rc::Rc;

use macroquad::window;

use crate::app;
use crate::app::state::textbox::{reducer, TextBoxAction, TextBoxState};
use crate::app::state::State;
use crate::app::{theme::Theme, BorderParams, Style, Value};

pub struct TextBox {
    pub style: Style,
    pub state: State<TextBoxState, TextBoxAction>,
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
                TextBoxState::new(text, Rc::clone(&style.theme.ghost)),
                reducer,
            ),
        }
    }

    pub fn on_type(&self, c: char) -> bool {
        let state = self.state.sub();
        if state.borrow().index == state.borrow().letters.len() - 1 {
            return true;
        }

        self.state.dispatch(TextBoxAction::TypeChar(
            c,
            Rc::clone(&self.style.theme.text),
            Rc::clone(&self.style.theme.ghost),
        ));

        false
    }

    pub fn delete_char(&self) {
        if self.state.sub().borrow().index == 0 {
            return;
        }

        self.state.dispatch(TextBoxAction::DeleteChar(Rc::clone(
            &self.style.theme.ghost,
        )));
    }

    fn update_position(&self, line_breaks: &[usize]) {
        let mut left: i32 = 0;
        let mut right: i32 = line_breaks.len() as i32 - 1;

        let state = self.state.sub();
        let index = state.borrow().index;

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

        if scroll != state.borrow().scroll {
            self.state.dispatch(TextBoxAction::Scroll(scroll));
        }
    }

    pub fn get_wpm(&self, end: Option<usize>) -> u16 {
        // thread::sleep(Duration::from_secs(1));
        let state = self.state.sub();
        let end = end.unwrap_or(state.borrow().letters.len());

        let time_passed: u128 = state.borrow().time_started.elapsed().as_millis();

        let mut wrongs = 0.0;
        let mut is_word_wrong = false;

        for i in 0..end {
            let letter = &state.borrow().letters[i];

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
        let state = self.state.sub();
        let t = state.borrow().timer.elapsed();

        if !state.borrow().started || t.as_millis() < 500 {
            return;
        }

        self.state.dispatch(TextBoxAction::AddWmp(
            self.get_wpm(Some(state.borrow().index)),
        ));
    }

    pub fn update(&self) {
        self.style.draw_bg();

        let state = self.state.sub();

        let line_breaks = app::text::print_letters_wrap(
            &self.style,
            &state.borrow().letters,
            state.borrow().index,
            state.borrow().scroll,
        );
        self.update_position(&line_breaks);

        self.style.draw_mask();

        if state.borrow().index > 0 && !state.borrow().started {
            self.state.dispatch(TextBoxAction::TimerStart);
        }
        self.get_incremental_wpm();
    }
}
