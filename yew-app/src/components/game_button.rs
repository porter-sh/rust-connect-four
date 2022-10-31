use yew::{Callback, function_component, html, Properties};
use yew_router::prelude::*;
use crate::router;
use gloo::console::error;

#[derive(Properties, PartialEq)]
pub struct Props {
    route: router::Route
}

#[function_component(GameButton)]
pub fn game_button(prop: &Props) -> Html {
    
    if let Some(history) = use_history() {
        let route = prop.route.clone();
        return html! {
            <button onclick={
                Callback::from(move |_| history.push(route.clone()))
            } />
        }
    }
    
    error!("Error Rendering Button");
    html! {
        <button>{ "Error Rendering Button" }</button>
    }

}