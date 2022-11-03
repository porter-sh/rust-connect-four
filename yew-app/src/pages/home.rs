use crate::components::game_button;
use crate::router;
use yew::prelude::*;

use gloo::console::log;
use gloo_net::websocket::futures::WebSocket;

#[function_component(Home)]
pub fn home() -> Html {
    if let Ok(ws) = WebSocket::open("ws://127.0.0.1:8081") {
        log!("connected");
    } else {
        log!("Failed to connect to server.");
    }

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
