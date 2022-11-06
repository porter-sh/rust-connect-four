//! game_button contains the GameButton component, which routes the app to a specified page

use crate::router::Route;
use gloo::console::error;
use yew::{function_component, html, Callback, Properties};
use yew_router::prelude::*;

/// Properties for what text the button should show, and which page to route to
#[derive(Properties, PartialEq)]
pub struct GameButtonProperties {
    pub text: &'static str,
    pub route: Route,
}

/// GameButton component
/// Routes to the specified page when clicked
#[function_component(GameButton)]
pub fn game_button(props: &GameButtonProperties) -> Html {
    if let Some(history) = use_history() {
        let route = props.route.clone();
        return html! {
            <button class="menu-btn" onclick={
                Callback::from(move |_| history.push(route.clone())) // route to the specified route when clicked
            }>{ props.text }</button>
        };
    }

    // An error likely occured because this GameButton is not the child of a router component
    error!("Error Rendering Button");
    html! {
        <button>{ "Error Rendering Button" }</button>
    }
}
