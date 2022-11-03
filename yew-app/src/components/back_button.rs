use crate::router;
use gloo::console::{error, log};
use yew::{function_component, html, Callback};
use yew_router::prelude::*;

#[function_component(BackButton)]
pub fn back_button() -> Html {
    if let Some(history) = use_history() {
        if let Some(route) = history.location().route::<router::Route>() {
            return html! {

                <button class="control-btn"
                    hidden={
                        route == router::Route::Home
                    }
                    onclick={
                        Callback::from(move |_| history.push(router::Route::Home))
                    }
                >
                    { "Quit Game" }
                </button>

            };
        }
    }

    error!("Error Rendering Button");
    html! {
        <button>{ "Error Rendering Button" }</button>
    }
}
