use wasm_bindgen::JsCast;
use wasm_bindgen::UnwrapThrowExt;
use web_sys::Event;
use web_sys::EventTarget;
use web_sys::FocusEvent;
use web_sys::HtmlInputElement;
use web_sys::InputEvent;
use web_sys::MouseEvent;

pub fn get_base_input_event(e: InputEvent) -> Event {
    e.dyn_into().unwrap_throw()
}

pub fn get_input_target(e: InputEvent) -> EventTarget {
    let event = get_base_input_event(e);
    event.target().unwrap_throw()
}

pub fn get_input_value(e: InputEvent) -> String {
    let target = get_input_target(e);
    let input_element: HtmlInputElement = target.dyn_into().unwrap_throw();
    input_element.value()
}

pub fn get_base_focus_event(e: FocusEvent) -> Event {
    e.dyn_into().unwrap_throw()
}

pub fn get_focus_target(e: FocusEvent) -> EventTarget {
    let event = get_base_focus_event(e);
    event.target().unwrap_throw()
}

pub fn prevent_default_on_focus(e: FocusEvent) {
    get_base_focus_event(e).prevent_default();
}

pub fn get_base_mouse_event(e: MouseEvent) -> Event {
    e.dyn_into().unwrap_throw()
}

pub fn prevent_default_on_mouse(e: MouseEvent) {
    get_base_mouse_event(e).prevent_default();
}
