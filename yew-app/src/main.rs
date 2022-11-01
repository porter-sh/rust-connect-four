#[allow(unused)]
use gloo::console::log;
use yew::prelude::*;
use yew_router::prelude::*;

pub mod components;
pub mod constants;
pub mod pages;
pub mod router;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <BrowserRouter>
                <components::back_button::BackButton />
                <Switch<router::Route> render={Switch::render(router::switch_route)} />
            </BrowserRouter>
        </>
    }
}

fn main() {
    // run with trunk serve
    yew::start_app::<App>();
}
