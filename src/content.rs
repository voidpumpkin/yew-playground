use web_sys::window;
use yew::prelude::*;
use yew_router::attach_route_listener;
use yew_router::prelude::*;

use crate::app::Route;

#[function_component(Content)]
pub fn content() -> Html {
    let pathname = use_state(|| window().unwrap().location().pathname().unwrap());
    let route = use_state(Route::current_route);

    let pathname_clone = pathname.clone();
    let route_clone = route.clone();

    use_state(move || {
        attach_route_listener(Callback::from(move |r: Option<Route>| {
            route_clone.set_if_neq(r);
            pathname_clone.set_if_neq(window().unwrap().location().pathname().unwrap());
        }))
    });

    html! {
        <div class="container">
            <div>{"Listener: "}{format!("{:?}", *route)}</div>
            <div>{"Pathname: "}{pathname.to_string()}</div>
            <div class="row">
                <Link<Route> route={Route::Home}>{"Home"}</Link<Route>>
                <Link<Route> route={Route::A}>{"A"}</Link<Route>>
                <Link<Route> route={Route::B}>{"B"}</Link<Route>>
            </div>
        </div>
    }
}
