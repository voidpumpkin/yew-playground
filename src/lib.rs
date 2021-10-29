mod selection_listener;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn start() {
    wasm_logger::init(wasm_logger::Config::default());

    yew::start_app::<selection_listener::SelectionListener>();
}
