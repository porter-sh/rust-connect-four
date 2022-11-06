//! back_button contains the BackButton component
//! 
//! BackButton automatically hides on the home page,
//! otherwise it brings the player back to the home page

use crate::router::Route;
use gloo::console::error;
use yew::{function_component, html, Callback};
use yew_router::prelude::*;

/// BackButton component
/// Brings the player to the home page
/// Hidden on the home page
#[function_component(BackButton)]
pub fn back_button() -> Html {
    if let Some(history) = use_history() {
        if let Some(route) = history.location().route::<Route>() {
            return html! {

                <button class=
                    {
                        if route == Route::Home {
                            "control-hidden"
                        } else {
                            "control-btn"
                        }
                    }
                    onclick={
                        Callback::from(move |_| history.push(Route::Home)) // route home when clicked
                    }
                >
                    { "Quit Game" }
                </button>

            };
        }
    }

    // An error likely occured because this BackButton is not the child of a router component
    error!("Error Rendering Button");
    html! {
        <button>{ "Error Rendering Button" }</button>
    }
}
