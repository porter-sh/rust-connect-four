//! lobby_select contains the LobbySelect component
//! LobbySelect renders a menu to select the desired lobby

use crate::{components::game_button::GameButton, router::Route};
use yew::prelude::*;

/// LobbySelect component
/// Displays a menu over the board to chose the desired lobby
#[function_component(LobbySelect)]
pub fn lobby_select() -> Html {
    html! {
        <>
            <div class="background-blur" />
            <div class={"menu-container"}>
                <p class="menu-txt">{"Choose Lobby"}</p>
                <form action={"/online-multiplayer/"}>
                    <label class={"menu-txt"} style={"font-size:15px"}
                            for="lobby">{"Lobby Name: "}</label>
                    <input type="text" name="lobby" id="lobby-input"
                            style={"text-align:center;"} placeholder={"(optional)"}/>
                    <input type="submit" value="Join" class="menu-btn"/>
                </form>
                <GameButton<Route> text={"Back"} route={Route::Home} />
            </div>
        </>
    }
}
