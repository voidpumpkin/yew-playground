use js_sys::Reflect;
use wasm_bindgen::UnwrapThrowExt;
use yew::prelude::*;

pub struct SelectionListener {}

pub enum Msg {
    Selected(usize),
}

impl Component for SelectionListener {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Selected(selection) => log::info!("{}", selection),
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let onchange = link.callback(|event: Event| {
            let target = event.target().unwrap_throw();
            let selection_js = Reflect::get(&target, &"selectedIndex".into()).unwrap_throw();
            let selection: usize = selection_js.into_serde().unwrap();
            Msg::Selected(selection)
        });
        html! {
            <select class="uk-select" {onchange} >
                <option value="volvo">{"Volvo"}</option>
                <option value="saab">{"Saab"}</option>
                <option value="mercedes">{"Mercedes"}</option>
                <option value="audi">{"Audi"}</option>
            </select>
        }
    }
}
