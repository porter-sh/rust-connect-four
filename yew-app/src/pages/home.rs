//! home contains the Home component for the home page
//! Home renders a menu to select the gamemode

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
use yew::prelude::*;

/// Home component
/// Displays a menu over the board to chose the desired gamemode
#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <>
            <div class="background-blur" />
            <div class={ "menu-container" }>
                <p class="menu-txt">{ "Menu" }</p>
                <GameButton<Route> text={ "Local Multiplayer" } route={Route::LocalMultiplayer} />
                <GameButton<Route> text={ "Online Multiplayer" } route={Route::LobbySelect} />
                <GameButton<Route> text={ "Singleplayer" } route={Route::AISelect} />
            </div>
            <footer class={ "footer" }>
                <a href={ "https://github.com/porter-sh/rust-connect-four" }>{ "source" }</a>
            </footer>
        </>
    }
}
