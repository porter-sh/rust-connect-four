use yew::{Html, html};
use yew_router::prelude::*;

use crate::pages::home::Home;

pub fn switch_route(route: &Route) -> Html {

    match route {

        Route::Home => html! {<Home />},
        Route::LocalMultiplayer => html! { {"TODO: Local Multiplayer"} },
        Route::VersusBot => html! { {"TODO: VersusBot"} },
        Route::OnlineMultiplayer => html! { {"TODO: Online Multiplayer"} },
        Route::NotFound => html! {
            { "This is not the page you are looking for :( LLLLL" }
        },
        Route::NotFoundNeedsRedirect => html! {
            <Redirect<Route> to={Route::NotFound} />
        }
    }
    
}

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("local-multiplayer")]
    LocalMultiplayer,
    #[at("versus-bot")]
    VersusBot,
    #[at("online-multiplayer")]
    OnlineMultiplayer,
    #[at("/404")]
    NotFound,
    #[not_found]
    #[at("/not_found")]
    NotFoundNeedsRedirect
}