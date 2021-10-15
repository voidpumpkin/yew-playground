use web_sys::window;
use yew::prelude::*;
use yew_router::attach_route_listener;
use yew_router::prelude::*;

use crate::app::Route;

fn get_path() -> String {
    window().unwrap().location().pathname().unwrap()
}

fn get_current_route_correctly() -> Option<Route> {
    Route::from_path(&get_path(), &Default::default())
}

#[function_component(Content)]
pub fn content() -> Html {
    let pathname = use_state(get_path);
    let route = use_state(get_current_route_correctly);

    {
        let pathname = pathname.clone();
        let route = route.clone();
        use_state(move || {
            attach_route_listener(Callback::from(move |r: Option<Route>| {
                let correct_route = get_current_route_correctly();

                log::info!(
                    "called callback\ngave: {:?}\nbut is: {:?}",
                    r.unwrap(),
                    correct_route.clone().unwrap()
                );

                route.set_if_neq(correct_route);
                pathname.set_if_neq(get_path());
            }))
        });
    }

    html! {
        <div class="container">
            <div>{"Route: "}{format!("{:?}", *route)}</div>
            <div>{"Pathname: "}{pathname.to_string()}</div>
            <div class="row">
                <Link<Route> route={Route::Home}>{"Home"}</Link<Route>>
                <Link<Route> route={Route::A}>{"A"}</Link<Route>>
                <Link<Route> route={Route::B}>{"B"}</Link<Route>>
            </div>
        </div>
    }
}
