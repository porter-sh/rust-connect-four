pub mod Router {

use yew_router::prelude::*;

pub fn switch_route(route: &Route) -> Html {

    match route {

        Route::Home => html! {},
        Route::NewGame => html! {},
        Route::InGame => html! {
            <Switch<InGameRoute> render={Switch::render(switch_ingame_route)} />
        },
        Route::NotFound => html! {
            { "This is not the page you are looking for :( LLLLL" }
        },


        _ => html! {}

    }

    html! {
        { "Hello" }
    }
    
}

pub fn switch_ingame_route(route: &InGameRoute) -> Html {

    html! {
        {"TODO"}
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
    #[not_found]
    #[at("/404")]
    NotFound
}

#[derive(Clone, Routable, PartialEq)]
pub enum InGameRoute {
    #[at("/game/local-multiplayer")]
    LocalMultiplayer,
    #[at("/game/versus-bot")]
    VersusBot,
    #[at("/game/online-multiplayer")]
    OnlineMultiplayer
}

}