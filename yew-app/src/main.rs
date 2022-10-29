use yew::prelude::*;

pub mod components;

#[function_component(App)]
fn app() -> Html {
    html! {
        <h1>{ "Hello World" }</h1>
    }
}

fn main() {
    // run with trunk serve
    yew::start_app::<App>();
}