use yew::prelude::*;

use crate::utils::{get_input_value,prevent_default_on_mouse};

#[function_component(App)]
pub fn app() -> Html {
    let email_handle = use_state(|| "".to_string());
    let email = (&*email_handle).clone();
    let footer_email = email.clone();

    let on_input = Callback::from(move |new_value| {
        email_handle.set(new_value);
    });

    let onclick = Callback::from(move |e| {
        prevent_default_on_mouse(e);
    });

    html! {
        <>
            <nav class="navbar navbar-dark bg-primary">
                <div class="container-fluid">
                    <a class="navbar-brand" href="/" {onclick}>{"Navbar"}</a>
                </div>
            </nav>
            <main class="container py-3">
                <EmailForm {on_input} {email} />
            </main>
            <footer class="p-5 bg-light">
                {footer_email}
            </footer>
        </>
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct EmailFormProps {
    pub on_input: Callback<String>,
    pub email: String,
}

#[function_component(EmailForm)]
pub fn email_form(props: &EmailFormProps) -> Html {
    let EmailFormProps { on_input, email } = props.clone();

    let error_handle = use_state(|| None::<String>);
    let error = (&*error_handle).clone();

    let oninput = Callback::from(move |e| {
        let new_value = get_input_value(e);
        on_input.emit(new_value);
    });

    let email_for_hook = email.clone();
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
        <div class="row">
            <label for="email" class="form-label">{"Email"}</label>
            <input type="email" class="form-control" id="email" placeholder="name@example.com" {email} {oninput} />
            <Error {error} />
        </div>
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
