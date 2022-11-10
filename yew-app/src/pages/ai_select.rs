//! ai_select contains the AiSelect component
//! AiSelect renders a menu to select the desired AI

use crate::{
    components::game_button::GameButton,
    router::{AiRoute, Route},
};
use yew::prelude::*;

/// AiSelect component
/// Displays a menu over the board to chose the desired AI
#[function_component(AiSelect)]
pub fn ai_select() -> Html {
    html! {
        <>
            <div class="background-blur" />
            <div class={"menu-container"}>
                <p class="menu-txt">{"Choose AI"}</p>
                <GameButton<AiRoute> text={"Random"} route={AiRoute::Random} />
                <GameButton<Route> text={"Back"} route={Route::Home} />
            </div>
        </>
    }
}
