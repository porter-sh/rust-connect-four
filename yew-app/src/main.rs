use yew::prelude::*;

pub mod components;
pub mod router;

#[function_component(App)]
fn app() -> Html {
    html! {
        <components::board::Board />
    }
}

fn main() {
    // run with trunk serve
    yew::start_app::<App>();
}