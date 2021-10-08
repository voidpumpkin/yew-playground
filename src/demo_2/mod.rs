use std::ops::Deref;

use yew::prelude::*;

use crate::utils::get_input_value;

pub struct App {
    email: String,
}

pub enum Msg {
    SetEmail(String),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            email: String::default(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SetEmail(email) => {
                self.email = email;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let on_input = link.callback(Msg::SetEmail);
        let email = self.email.clone();
        let footer_email = email.clone();
        html! {
            <>
                <nav class="navbar navbar-dark bg-primary">
                    <div class="container-fluid">
                        <a class="navbar-brand" href="#">{"Navbar"}</a>
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
}

#[derive(Properties, PartialEq, Clone)]
pub struct EmailFormProps {
    pub on_input: Callback<String>,
    pub email: String,
}

#[function_component(EmailForm)]
pub fn email_form(props: &EmailFormProps) -> Html {
    let EmailFormProps {
        on_input,
        email: value,
    } = props.clone();
    let email = value.clone();

    let error_handle = use_state(|| None::<String>);
    let error = error_handle.deref().clone();

    use_effect(move || {
        let banned_word_error = Some("Banned word has been used!!!".to_string());
        let is_using_banned_word = email == "error";
        if is_using_banned_word && *error_handle != banned_word_error {
            error_handle.set(banned_word_error);
        } else if !is_using_banned_word && *error_handle == banned_word_error {
            error_handle.set(None);
        }
        || ()
    });

    let oninput = Callback::from(move |e| {
        let new_value = get_input_value(e);
        on_input.emit(new_value)
    });

    html! {
        <div class="row">
            <label for="email" class="form-label">{"Email"}</label>
            <input
                type="email"
                class="form-control"
                id="email"
                placeholder="name@example.com"
                {value}
                {oninput}
            />
            <Error {error}/>
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
