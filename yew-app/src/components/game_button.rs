use crate::router;
use gloo::console::error;
use yew::{function_component, html, Callback, Properties};
use yew_router::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub text: &'static str,
    pub route: router::Route,
}

#[function_component(GameButton)]
pub fn game_button(props: &Props) -> Html {
    if let Some(history) = use_history() {
        let route = props.route.clone();
        return html! {
            <button class="menu-btn" onclick={
                Callback::from(move |_| history.push(route.clone()))
            }>{ props.text }</button>
        };
    }

    error!("Error Rendering Button");
    html! {
        <button>{ "Error Rendering Button" }</button>
    }
}
