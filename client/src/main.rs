use yew::prelude::*;
use yew_router::prelude::*;
use web_sys;

use client::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
        <BrowserRouter>
            <Header/>
            <Switch<MainRoute> render={switch} /> // <- must be child of <BrowserRouter>
        </BrowserRouter>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
    let yes = utils::requests::Method::POST;
}
