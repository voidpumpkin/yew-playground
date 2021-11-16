mod portal;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn start() {
    // println!("hi");
    // web_sys::console::log_1(&"log".into());
    wasm_logger::init(wasm_logger::Config::default());
    // log::info!("test");
    // yew::start_app::<counter::Counter>();
    // yew::start_app::<counter_old::CounterOld>();
    // yew::start_app::<counter_reducer::CounterReducer>();
    // yew::start_app::<counter_hook::CounterHook>();
    yew::start_app::<portal::Model>();
}
