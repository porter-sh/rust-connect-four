use yew::prelude::*;
use yew_router::prelude::*;

pub mod components;
pub mod constants;
pub mod pages;
pub mod router;
pub mod util;

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <components::back_button::BackButton />
            <Switch<router::Route> render={Switch::render(router::switch_route)} />
            <components::board::Board />
        </BrowserRouter>
    }
}

fn main() {
    // run with trunk serve
    yew::start_app::<App>();
}
