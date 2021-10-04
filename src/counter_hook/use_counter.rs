use std::rc::Rc;
use yew::prelude::*;

pub enum Action {
    Add,
    #[allow(unused)]
    SetIsCounting(bool),
}

#[derive(Clone)]
pub struct CounterState {
    pub value: i32,
    #[allow(unused)]
    pub is_counting: bool,
}

pub fn use_counter<T, F: FnOnce() -> i32>(steps_init: F) -> (i32, Callback<T>) {
    let steps_ref = use_ref(steps_init);

    let counter_state = use_reducer(
        // the reducer function
        move |prev: Rc<CounterState>, action: Action| match action {
            Action::Add => CounterState {
                value: prev.value + *steps_ref.borrow(),
                ..*prev
            },
            Action::SetIsCounting(is_counting) => CounterState {
                is_counting,
                ..*prev
            },
        },
        // initial state
        CounterState {
            value: 0,
            is_counting: false,
        },
    );

    let add_cb = {
        let counter_state = counter_state.clone();
        Callback::from(move |_| counter_state.dispatch(Action::Add))
    };

    (counter_state.value, add_cb)
}
