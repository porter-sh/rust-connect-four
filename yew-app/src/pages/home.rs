//! home contains the Home component for the home page
//! Home renders a menu to select the gamemode

use crate::{components::game_button::GameButton, router::Route};
use yew::prelude::*;

/// Home component
/// Displays a menu over the board to chose the desired gamemode
#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <>
            <div class="background-blur" />
            <div class={"menu-container"}>
                <p class="menu-txt">{"Menu"}</p>
                <GameButton<Route> text={"Local Multiplayer"} route={Route::LocalMultiplayer} />
                <GameButton<Route> text={"Online Multiplayer"} route={Route::LobbySelect} />
                <GameButton<Route> text={"Singleplayer"} route={Route::AISelect} />
            </div>
        </>
    }
}
