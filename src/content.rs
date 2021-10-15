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

fn get_current_route_correctly() -> Option<Route> {
    Route::from_path(&get_current_pathname(), &Default::default())
}

fn create_route_listener(route: UseStateHandle<Option<Route>>) -> RouteListener {
    //for some reason new_route always one behind
    attach_route_listener(Callback::from(move |new_route: Option<Route>| {
        let r = get_current_route_correctly();
        log::info!(
            "event listener triggered\ngot: {:?}\nbut is:{:?}",
            new_route.unwrap(),
            r.clone().unwrap()
        );
        route.set_if_neq(r);
    }))
}

#[function_component(Content)]
pub fn content() -> Html {
    let route = use_state(Route::current_route);
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
