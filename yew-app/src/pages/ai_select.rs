//! ai_select contains the AiSelect component
//! AiSelect renders a menu to select the desired AI

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
                <GameButton<AIRoute> text={"Perfect"} route={AIRoute::Perfect} />
                <GameButton<AIRoute> text={"Survival"} route={AIRoute::Survival} />
                <GameButton<Route> text={"Back"} route={Route::Home} />
            </div>
        </>
    }
}
