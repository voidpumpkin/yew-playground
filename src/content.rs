use web_sys::window;
use yew::prelude::*;
use yew_router::attach_route_listener;
use yew_router::prelude::*;
use yew_router::RouteListener;

use crate::app::Route;
use crate::use_memo::use_memo;

fn get_current_pathname() -> String {
    window().unwrap().location().pathname().unwrap()
}

fn create_route_listener(route: UseStateHandle<Option<Route>>) -> RouteListener {
    log::info!("create_route_listener {:?}", route);

    attach_route_listener(Callback::from(move |new_route: Option<Route>| {
        log::info!("agent {:?}", new_route); //for some reason always one click behind
        route.set(new_route);
    }))
}

#[function_component(Content)]
pub fn content() -> Html {
    let route = use_state(Route::current_route);
    log::info!("rerender {:?}", route);

    let route_for_cb = route.clone();
    let route_for_listner = route.clone();
    let _route_listener = use_memo(
        move || create_route_listener(route_for_cb),
        route_for_listner,
    );

    let route_for_pathname = route.clone();
    let pathname = use_memo(get_current_pathname, route_for_pathname);

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
