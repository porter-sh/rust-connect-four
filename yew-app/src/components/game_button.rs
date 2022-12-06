//! game_button contains the GameButton component, which routes the app to a specified page

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

use yew::{function_component, html, Callback, Html, Properties};
use yew_router::prelude::*;

use gloo::console::error;

/// Properties for what text the button should show, and which page to route to
#[derive(Properties, PartialEq)]
pub struct GameButtonProperties<T: Routable> {
    pub text: &'static str,
    pub route: T,
}

/// GameButton component
/// Routes to the specified page when clicked
#[function_component(GameButton)]
pub fn game_button<T>(props: &GameButtonProperties<T>) -> Html
where
    T: Routable + 'static,
{
    if let Some(navigator) = use_navigator() {
        let route = props.route.clone();
        return html! {
            <button class="menu-btn" onclick={
                Callback::from(move |_| navigator.push(&route)) // route to the specified route when clicked
            }>{ props.text }</button>
        };
    }

    // An error likely occured because this GameButton is not the child of a router component
    error!("Error Rendering Button");
    html! {
        <button>{ "Error Rendering Button" }</button>
    }
}
