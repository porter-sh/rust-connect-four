use crate::components::game_button;
use crate::router;
use yew::prelude::*;

use gloo::console::log;
use std::io::Read;
use std::net::TcpStream;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div class={"menu-container"}>
            <p class="menu-txt">{"Menu"}</p>
            <game_button::GameButton text={"Local Multiplayer"} route={router::Route::LocalMultiplayer} />
            <game_button::GameButton text={"Online Multiplayer"} route={router::Route::OnlineMultiplayer} />
            <game_button::GameButton text={"Singleplayer"} route={router::Route::VersusBot} />
            <p>
                {
                    if let Ok(mut stream) = TcpStream::connect("127.0.0.1:8081") {
                        let mut message = String::new();
                        match stream.read_to_string(&mut message) {
                            Ok(_) => message,
                            Err(e) => "Error reading string".to_string(),
                        }
                    } else {
                        "Failed to connect to server.".to_string()
                    }
                }
            </p>
        </div>
    }
}
