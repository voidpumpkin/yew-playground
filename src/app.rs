use yew::prelude::*;
use yew_router::prelude::*;

use crate::content::Content;

#[derive(Routable, Debug, Clone, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/a")]
    A,
    #[at("/b")]
    B,
}

#[function_component(App)]
pub fn app() -> Html {
    let render_route = Router::render(|_| {
        html! {<Content />}
    });

    html! {<Router<Route> render={render_route} />}
}
