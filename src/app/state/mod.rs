use std::cell::{Ref, RefCell};
use std::rc::Rc;

pub mod screen;
pub mod textbox;
pub mod typing_test;

pub struct State<StateType, ActionType> {
    val: Rc<RefCell<StateType>>,
    reducer: fn(Rc<RefCell<StateType>>, ActionType),
}

impl<StateType, ActionType> State<StateType, ActionType> {
    pub fn new(initial_state: StateType, reducer: fn(Rc<RefCell<StateType>>, ActionType)) -> Self {
        State {
            val: Rc::new(RefCell::new(initial_state)),
            reducer,
        }
    }

    pub fn dispatch(&self, action: ActionType) {
        (self.reducer)(Rc::clone(&self.val), action);
    }

    //pub fn get(&self) -> Ref<'_, StateType> {
    //    self.val.borrow()
    //}

    pub fn sub(&self) -> Rc<RefCell<StateType>> {
        Rc::clone(&self.val)
    }
}
