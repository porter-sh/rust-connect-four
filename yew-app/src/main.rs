//     ,,========..        ,,========..        ,,========..        ,,========..
//    //  ,----.  \\      //  ,----.  \\      //  ,----.  \\      //  ,----.  \\
//   //  / ,--. \  \\    //  / ,--. \  \\    //  / ,--. \  \\    //  / ,--. \  \\
//  ||  ! |    | !  ||  ||  ! |    | !  ||  ||  ! |    | !  ||  ||  ! |    | !  ||
//   \\  \ `--' /  //    \\  \ `--' /  //    \\  \ `--' /  //    \\  \ `--' /  //
//    \\  `----'  //      \\  `----'  //      \\  `----'  //      \\  `----'  //
//     ``========''        ``========''        ``========''        ``========''

use yew::prelude::*;
use yew_router::prelude::*;

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
            <Switch<router::Route> render={Switch::render(router::switch_route)} /> // renders additional page specific components / logic
            <components::board::Board />
        </BrowserRouter>
    }
}

fn main() {
    // run with trunk serve
    yew::start_app::<App>();
}
