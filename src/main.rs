mod counter;
mod counter_hook;
mod counter_old;
mod counter_reducer;
mod demo_2;
mod demo_2_prep;
mod selection_listener;
pub mod utils;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    // yew::start_app::<counter::Counter>();
    // yew::start_app::<counter_old::CounterOld>();
    // yew::start_app::<counter_reducer::CounterReducer>();
    // yew::start_app::<counter_hook::CounterHook>();
    // yew::start_app::<selection_listener::SelectionListener>();
    // yew::start_app::<demo_2_prep::App>();
    yew::start_app::<demo_2::App>();
}
