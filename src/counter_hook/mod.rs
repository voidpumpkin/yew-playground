use yew::prelude::*;

mod use_counter;

use use_counter::use_counter;

#[function_component(CounterHook)]
pub fn counter_hook() -> Html {
    html! {
        <>
            <InnerOne />
            <InnerTwo />
        </>
    }
}

#[function_component(InnerOne)]
pub fn inner_one() -> Html {
    let (value, onclick) = use_counter(|| 1);

    html! {
        <div>
            <button {onclick}>{ "+1" }</button>
            <p>{ value }</p>
        </div>
    }
}

#[function_component(InnerTwo)]
pub fn inner_two() -> Html {
    let (value, onclick) = use_counter(|| 2);

    html! {
        <div>
            <button {onclick}>{ "+2" }</button>
            <p>{ value }</p>
        </div>
    }
}
