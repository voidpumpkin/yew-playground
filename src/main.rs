use gloo::utils::document;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::{Event, HtmlInputElement, InputEvent};
use yew::prelude::*;

pub fn get_input_value(input_event: InputEvent) -> String {
    let event: Event = input_event.dyn_into().unwrap_throw();
    get_input_value_from_event(event)
}

pub fn get_input_value_from_event(event: Event) -> String {
    let target = event.target().unwrap_throw();
    let input_element: HtmlInputElement = target.dyn_into().unwrap_throw();
    input_element.value()
}

#[function_component]
fn App() -> Html {
    let value_handle = use_state(String::default);
    let value = (*value_handle).clone();

    let oninput = {
        Callback::from(move |e: InputEvent| {
            value_handle.set(get_input_value(e));
        })
    };

    let modal_root = document()
        .get_element_by_id("modal-root")
        .expect("Expected to find a #modal-root element");

    create_portal(
        html! {
            <input
                type="text"
                placeholder="Type here"
                {value}
                {oninput}
            />
        },
        modal_root,
    )
}

pub fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    let root = document()
        .get_element_by_id("root")
        .expect("Expected to find a #root element");

    yew::Renderer::<App>::with_root(root).render();
}
