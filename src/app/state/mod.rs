pub mod screen;
pub mod textbox;
pub mod typing_test;

pub struct State<StateType, ActionType> {
    val: StateType,
    reducer: fn(&StateType, ActionType),
}

impl<StateType, ActionType> State<StateType, ActionType> {
    pub fn new(initial_state: StateType, reducer: fn(&StateType, ActionType)) -> Self {
        State {
            val: initial_state,
            reducer,
        }
    }

    pub fn dispatch(&self, action: ActionType) {
        (self.reducer)(&self.val, action);
    }

    pub fn get(&self) -> &StateType {
        &self.val
    }
}
