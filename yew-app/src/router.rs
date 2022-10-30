use yew::{Html, html};
use yew_router::prelude::*;

use crate::pages::home::Home;

pub fn switch_route(route: &Route) -> Html {

    match route {

        Route::Home => html! {<Home />},
        Route::NewGame => html! {},
        Route::InGame => html! {
            <Switch<InGameRoute> render={Switch::render(switch_ingame_route)} />
        },
        Route::NotFound => html! {
            { "This is not the page you are looking for :( LLLLL" }
        },
        Route::NotFoundNeedsRedirect => html! {
            <Redirect<Route> to={Route::NotFound} />
        }
    }
    
}

pub fn switch_ingame_route(route: &InGameRoute) -> Html {

    match route {
        InGameRoute::LocalMultiplayer => html! { {"TODO: Local Multiplayer"} },
        InGameRoute::VersusBot => html! { {"TODO: VersusBot"} },
        InGameRoute::OnlineMultiplayer => html! { {"TODO: Online Multiplayer"} },
        InGameRoute::NotFound => html! {
            <Redirect<Route> to={Route::NotFound} />
        }
    }

}

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/new-game")]
    NewGame,
    #[at("/game/:s")]
    InGame,
    #[at("/404")]
    NotFound,
    #[not_found]
    #[at("/not_found")]
    NotFoundNeedsRedirect
}

#[derive(Clone, Routable, PartialEq)]
pub enum InGameRoute {
    #[at("/game/local-multiplayer")]
    LocalMultiplayer,
    #[at("/game/versus-bot")]
    VersusBot,
    #[at("/game/online-multiplayer")]
    OnlineMultiplayer,
    #[not_found]
    #[at("/game/404")]
    NotFound
}