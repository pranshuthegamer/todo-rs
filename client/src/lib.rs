use serde::Serialize;
use wasm_bindgen::UnwrapThrowExt;
use yew::{prelude::*, callback};
use web_sys;

use yew_router::prelude::*;

pub mod utils;

// Used to switch between the Main Routes
pub fn switch(routes: MainRoute) -> Html {
    match routes {
        MainRoute::Home => html! { <h1>{ "Home" }</h1> },
        MainRoute::Auth => html! {
            <Auth />
        },
        MainRoute::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

// The top level routes
#[derive(Routable, PartialEq, Clone)]
pub enum MainRoute {
    #[at("/")]
    Home,
    #[at("/auth")]
    Auth,
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
            <Link<MainRoute> to={MainRoute::Auth} classes="header-link">{"Login"}</Link<MainRoute>>
            <Link<MainRoute> to={MainRoute::Auth} classes="header-link">{"Register"}</Link<MainRoute>>
        </div>
    }
}


// function to render crate::MainRoute::Auth route
#[function_component(Auth)]
pub fn render_auth() -> Html {
    let count = use_state(|| 0);
    let onclick = Callback::from(move |_| {
        let count = count.clone();
        let greeting = format!("Hi there {}", *count);
        count.set(*count + 1);
        web_sys::console::log_1(&greeting.into())
    });
    html!(
        <>
        <div class="center auth main-containers" id="auth-container">
            <div class="center" style="margin-bottom: 0.5em;">{"enter sign in details"}</div>
            <input type="text" class="input center" id="input-username" placeholder="username"/>
            <input type="text" class="input center" id="input-password" placeholder="password"/>
            <button {onclick}>{"done"}</button>
        </div>
        </>
    )
}
