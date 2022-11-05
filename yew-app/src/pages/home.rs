use crate::components::game_button;
use crate::router;
use yew::prelude::*;

use futures::{SinkExt, StreamExt};
use gloo::console::log;
use gloo_net::websocket::{futures::WebSocket, Message};
use wasm_bindgen_futures::spawn_local;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <>
            <div class="background-blur" />
            <div class={"menu-container"}>
                <p class="menu-txt">{"Menu"}</p>
                <game_button::GameButton text={"Local Multiplayer"} route={router::Route::LocalMultiplayer} />
                <game_button::GameButton text={"Online Multiplayer"} route={router::Route::OnlineMultiplayer} />
                <game_button::GameButton text={"Singleplayer"} route={router::Route::VersusBot} />
            </div>
        </>
    }
}
