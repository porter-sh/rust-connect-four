//! ai_select contains the AiSelect component
//! AiSelect renders a menu to select the desired AI

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

use crate::{
    components::game_button::GameButton,
    router::{AIRoute, Route},
};
use yew::prelude::*;

/// AiSelect component
/// Displays a menu over the board to chose the desired AI
#[function_component(AISelect)]
pub fn ai_select() -> Html {
    html! {
        <>
            <div class="background-blur" />
            <div class={"menu-container"}>
                <p class="menu-txt">{"Select AI"}</p>
                <GameButton<AIRoute> text={"Random"} route={AIRoute::Random} />
                <GameButton<AIRoute> text={"Brute Force"} route={AIRoute::BruteForce} />
                <GameButton<AIRoute> text={"Survival"} route={AIRoute::Survival} />
                <GameButton<Route> text={"Back"} route={Route::Home} />
            </div>
        </>
    }
}
