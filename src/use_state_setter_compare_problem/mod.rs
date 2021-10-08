use std::ops::Deref;

use yew::prelude::*;

use crate::utils::get_input_value;

#[function_component(App)]
pub fn app() -> Html {
    let email_handle = use_state(|| "".to_string());
    let email = email_handle.deref().clone();
    let email_for_hook = email.clone();

    let error_handle = use_state(|| None::<String>);
    let error = error_handle.deref().clone();

    let oninput = Callback::from(move |e| {
        let new_value = get_input_value(e);
        email_handle.set(new_value);
    });

    // how it should work: (though it causes infinite loop)
    // use_effect(move || {
    //     let banned_word_error = Some("Banned word has been used!!!".to_string());
    //     let is_using_banned_word = email_for_hook == *"error";
    //     if is_using_banned_word {
    //         error_handle.set(banned_word_error);
    //     } else {
    //         error_handle.set(None); // setter triggers rerender even though i set same value as it was
    //     }
    //     || ()
    // });

    // but to work it has to be like this:
    use_effect(move || {
        let banned_word_error = Some("Banned word has been used!!!".to_string());
        let is_using_banned_word = email_for_hook == *"error";
        if is_using_banned_word && *error_handle != banned_word_error {
            error_handle.set(banned_word_error);
        } else if !is_using_banned_word && *error_handle != None {
            error_handle.set(None);
        }
        || ()
    });

    html! {
        <>
            <main class="container py-3">
                <div class="row">
                    <label for="email" class="form-label">{"Email"}</label>
                    <input type="email" class="form-control" id="email" placeholder="name@example.com" {email} {oninput} />
                    <Error {error} />
                </div>
            </main>
        </>
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct ErrorProps {
    pub error: Option<String>,
}

#[function_component(Error)]
pub fn error(props: &ErrorProps) -> Html {
    let ErrorProps { error } = props;
    if let Some(err) = error {
        html! { <div class="bg-danger">{err}</div> }
    } else {
        html! {}
    }
}
