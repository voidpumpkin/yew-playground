pub mod use_forever_suspension;

use gloo::timers::callback::Timeout;
use use_forever_suspension::use_forever_suspension;
use yew::prelude::*;

#[function_component]
fn FirstPage() -> HtmlResult {
    use_forever_suspension()?;

    Ok(html! {
        <button class="btn block">{ "Some button" }</button>
    })
}

#[function_component]
fn App() -> Html {
    let page_handle = use_state(|| 1);
    let page = *page_handle;

    use_memo(
        move |()| {
            Timeout::new(1_000, move || {
                page_handle.set(2);
            })
        },
        (),
    );

    use_memo(
        |page| {
            log::info!("🚀 {page}");
        },
        page,
    );

    html! {
        <>
        <Suspense fallback={html! {<div>{"Loading..."}</div>}}>
            if page == 1 {
                // If uncommented will work
                // <Suspense fallback={html! {<div>{"Loading 1..."}</div>}}>
                    <FirstPage />
                // </Suspense>
            } else {
                <h1>{"Page 2"}</h1>
            }
        </Suspense>
        </>
    }
}

pub fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    yew::Renderer::<App>::new().render();
}
