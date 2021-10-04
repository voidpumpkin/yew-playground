use yew_router::prelude::*;

#[derive(Routable, Debug, Clone, PartialEq)] //Err the trait bound `SubRoute: FromStr` is not satisfied
pub enum MyRoute {
    #[at("/some/route/:sub_route")]
    SubRoute { sub_route: SubRoute }, //Err the trait `std::fmt::Display` is not implemented for `SubRoute`
}

#[derive(Routable, Debug, Clone, PartialEq)]
pub enum SubRoute {
    #[at("/super/route")]
    SubSubRoute,
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
}
