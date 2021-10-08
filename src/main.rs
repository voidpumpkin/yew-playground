mod use_state_setter_compare_problem;
pub mod utils;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    yew::start_app::<use_state_setter_compare_problem::App>();
}
