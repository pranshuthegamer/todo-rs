use serde::de::value;
use yew::prelude::*;
use web_sys::{
    self, HtmlInputElement
};

use yew_router::{prelude::*, utils::fetch_base_url};

pub mod utils;

// Used to switch between the Main Routes
pub fn switch(routes: MainRoute) -> Html {
    match routes {
        MainRoute::Home => html! { <h1>{ "Home" }</h1> },
        MainRoute::AuthSignUp => html! {
            <AuthSignUp />
        },
        MainRoute::AuthSignIn => html! {
            <div>{"meow"}</div>
        },
        MainRoute::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

// The top level routes
#[derive(Routable, PartialEq, Clone)]
pub enum MainRoute {
    #[at("/")]
    Home,
    #[at("/auth/signup")]
    AuthSignUp,
    #[at("/auth/signin")]
    AuthSignIn,
    #[not_found]
    #[at("/404")]
    NotFound,
}


// function to render Header
#[function_component(Header)]
pub fn header() -> Html{
    html!{
        <div id="header">
            <Link<MainRoute> to={MainRoute::Home} classes="header-link">{"Home"}</Link<MainRoute>>
            <Link<MainRoute> to={MainRoute::AuthSignIn} classes="header-link">{"Sign In"}</Link<MainRoute>>
            <Link<MainRoute> to={MainRoute::AuthSignUp} classes="header-link">{"Sign Up"}</Link<MainRoute>>
        </div>
    }
}


// function to render crate::MainRoute::Auth route
#[function_component(AuthSignUp)]
pub fn render_auth_sign_up() -> Html {
    let username = use_state(|| String::from(""));
    let username_clone = username.clone();
    let password = use_state(|| String::from(""));
    let password_clone = password.clone();

    let onclick = Callback::from(move |_| {
        let username = (*username_clone).clone();
        let password = (*password_clone).clone();
        let req = common::schemas::UserRegister {
            username,
            password, 
            email: "yo".to_string()
        };
        
    });

    html!(
        <div class="center auth main-containers" id="auth-container">
            <div class="center" style="margin-bottom: 0.5em;">{"enter sign in details"}</div>
            <MyInput value={username} r#type="text" class="input center" id="input-username" placeholder="username"/>
            <MyInput value={password} r#type="password" class="input center" id="input-password" placeholder="password"/>
            <button {onclick}>{"done"}</button>
        </div>
    )
}


#[derive(Properties, PartialEq)]
pub struct MyInputProps {
    value: UseStateHandle<String>,
    #[prop_or_default]
    id: String,
    #[prop_or_default]
    r#type: String,
    #[prop_or_default]
    class: String,
    #[prop_or_default]
    placeholder: String,
}


#[function_component]
pub fn MyInput(props: &MyInputProps) -> Html {
    let input_node_ref = use_node_ref();

    let input_value_handle = props.value.clone();
    let input_value = (*input_value_handle).clone();

    let onchange = {
        let input_node_ref = input_node_ref.clone();

        Callback::from(move |_| {
            let input = input_node_ref.cast::<HtmlInputElement>();

            if let Some(input) = input {
                input_value_handle.set(input.value());
            }
        })
    };

    html! {
        <input ref={input_node_ref}
            {onchange}
            id={props.id.clone()}
            type={props.r#type.clone()}
            class={props.class.clone()}
            placeholder={props.placeholder.clone()}
            value={input_value}
        />
    }
}
