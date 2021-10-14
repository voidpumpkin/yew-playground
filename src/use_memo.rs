use yew::prelude::*;
/// Creates a memoized value
/// Or could be named use_callback since they both are kinda same in react
pub fn use_memo<T, F, Dependents>(create_value_fn: F, deps: Dependents) -> UseStateHandle<T>
where
    T: 'static,
    F: FnOnce() -> T + Clone + 'static,
    Dependents: PartialEq + 'static,
{
    let value_handle = use_state(create_value_fn.clone());

    let value_handle_clone = value_handle.clone();
    use_effect_with_deps(
        move |_| {
            value_handle_clone.set(create_value_fn());
            || {}
        },
        deps,
    );

    value_handle
}
