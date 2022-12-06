//     ,,========..        ,,========..        ,,========..        ,,========..
//    //  ,----.  \\      //  ,----.  \\      //  ,----.  \\      //  ,----.  \\
//   //  / ,--. \  \\    //  / ,--. \  \\    //  / ,--. \  \\    //  / ,--. \  \\
//  ||  ! |    | !  ||  ||  ! |    | !  ||  ||  ! |    | !  ||  ||  ! |    | !  ||
//   \\  \ `--' /  //    \\  \ `--' /  //    \\  \ `--' /  //    \\  \ `--' /  //
//    \\  `----'  //      \\  `----'  //      \\  `----'  //      \\  `----'  //
//     ``========''        ``========''        ``========''        ``========''

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

///// TODO LIST /////
///
/// Improve AI efficiency / moves looked ahead
///
/// run.md
///
/// Deploy the code somewhere
///
use yew::prelude::*;
use yew_router::prelude::*;

pub mod ai;
pub mod components;
pub mod pages;
pub mod router;
pub mod util;

/// Main application
/// Some logic is handled both by the individual page, as determined by the router
/// The Board and BackButton components also read the current route to determine state, and act accordingly
#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<router::Route> render={router::switch_route} /> // renders additional page specific components / logic
            <components::board::Board />
        </BrowserRouter>
    }
}

fn main() {
    // run with trunk serve
    yew::Renderer::<App>::new().render();
}
