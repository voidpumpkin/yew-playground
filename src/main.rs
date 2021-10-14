mod app;
mod content;
mod use_memo;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    yew::start_app::<app::App>();
}
