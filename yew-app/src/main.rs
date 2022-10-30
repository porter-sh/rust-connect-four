use yew::prelude::*;
use yew_router::prelude::*;
use gloo::console::log;

pub mod components;
pub mod router;
pub mod pages;

#[function_component(App)]
fn app() -> Html {
    log!("Here");
    html! {
        <BrowserRouter>
            <Switch<router::Route> render={Switch::render(router::switch_route)} />
        </BrowserRouter>
    }
}

fn main() {
    // run with trunk serve
    yew::start_app::<App>();
}