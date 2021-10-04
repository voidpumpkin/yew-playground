mod counter;
mod counter_hook;
mod counter_old;
mod counter_reducer;
mod selection_listener;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    // yew::start_app::<counter::Counter>();
    // yew::start_app::<counter_old::CounterOld>();
    // yew::start_app::<counter_reducer::CounterReducer>();
    // yew::start_app::<counter_hook::CounterHook>();
    yew::start_app::<selection_listener::SelectionListener>();
}
