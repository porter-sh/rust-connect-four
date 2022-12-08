//! lobby_select contains the LobbySelect component
//! LobbySelect renders a menu to select the desired lobby

/*
 * This file is part of Rust-Connect-Four
 * Copyright (C) 2022 Alexander Broihier <alexanderbroihier@gmail.com>
 * Copyright (C) 2022 Porter Shawver <portershawver@gmail.com>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published
 * by the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

use crate::{components::game_button::GameButton, router::Route};
use gloo::{console::error, utils::document};
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
                <form action={{
                    if let Ok(Some(base_uri)) = document().base_uri() {
                        base_uri + "online-multiplayer/"
                    } else {
                        error!("Error getting base uri");
                        "/online-multiplayer/".to_string()
                    }
                }}>
                    <label class={"menu-txt"} style={"font-size:15px"}
                            for="lobby">{"Lobby Name: "}</label>
                    <input type="text" name="lobby" id="lobby-input"
                            style={"text-align:center;"}
                            placeholder={"(optional)"} maxlength={"16"}/>
                    <input type="submit" value="Join" class="menu-btn"/>
                </form>
                <GameButton<Route> text={"Back"} route={Route::Home} />
            </div>
        </>
    }
}
