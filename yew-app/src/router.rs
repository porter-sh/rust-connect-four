//! router contains the various routes the application can go to
//!
//! router also contains a switch_route function to selectively render page specific components / logic

use crate::pages::{ai_select::AiSelect, home::Home, lobby_select::LobbySelect};
use yew::{html, Html};
use yew_router::prelude::*;

/// Render additional page specific components / logic
/// To be called in a Yew <Switch<Route> render={Switch::render(switch_route)} /> component
pub fn switch_route(route: &Route) -> Html {
    match route {
        Route::Home => html! {<Home />},
        Route::LobbySelect => html! {<LobbySelect />},
        Route::AiSelect => html! {<AiSelect />},
        Route::LocalMultiplayer => html! {},
        Route::VersusBot => html! { <Switch<AiRoute> render={Switch::render(switch_ai_route)} />},
        Route::OnlineMultiplayer => html! {},
        Route::NotFound => html! {
            { "This is not the page you are looking for :( LLLLL" }
        },
        Route::NotFoundNeedsRedirect => html! {
            <Redirect<Route> to={Route::NotFound} /> // force URL to show 404 for not found pages
        },
    }
}

pub fn switch_ai_route(route: &AiRoute) -> Html {
    match route {
        AiRoute::Random => html! {},
    }
}

/// Enum containing the routes (pages) the app can go to
#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/lobby-select")]
    LobbySelect,
    #[at("/ai-select")]
    AiSelect,
    #[at("/local-multiplayer")]
    LocalMultiplayer,
    #[at("/versus-bot/*")]
    VersusBot,
    #[at("/online-multiplayer/")]
    OnlineMultiplayer,
    #[at("/404")]
    NotFound,
    #[not_found]
    #[at("/not_found")]
    NotFoundNeedsRedirect, // force URL to show 404 (rather than the typed URL) for not found pages
}

#[derive(Clone, Routable, PartialEq)]
pub enum AiRoute {
    #[at("/versus-bot/random")]
    Random,
}
