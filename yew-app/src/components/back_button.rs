use yew::{Callback, function_component, html};
use yew_router::prelude::*;
use crate::router;
use gloo::console::{log, error};

#[function_component(BackButton)]
pub fn back_button() -> Html {
    
    if let Some(history) = use_history() {
        log!("Got this far.");
        if let Some (route) = history.location().route::<router::Route>() {
            return html! {

                <button
                    hidden={
                        route == router::Route::Home
                    }
                    onclick={
                        Callback::from(move |_| history.push(router::Route::Home))
                    }
                />

            }
        }
    }
    
    error!("Error Rendering Button");
    html! {
        <button>{ "Error Rendering Button" }</button>
    }

}