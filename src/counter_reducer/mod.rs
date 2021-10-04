use std::rc::Rc;

use yew::prelude::*;

#[function_component(CounterReducer)]
pub fn counter_reducer() -> Html {
    enum Action {
        Add,
        #[allow(unused)]
        SetIsCounting(bool),
    }

    struct CounterState {
        value: i32,
        #[allow(unused)]
        is_counting: bool,
    }

    let counter_state = use_reducer(
        // the reducer function
        |prev: Rc<CounterState>, action: Action| match action {
            Action::Add => CounterState {
                value: prev.value + 1,
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

    let onclick = {
        let counter_state = counter_state.clone();
        Callback::from(move |_| counter_state.dispatch(Action::Add))
    };

    html! {
        <div>
            <button {onclick}>{ "+1" }</button>
            <p>{ counter_state.value }</p>
        </div>
    }
}
