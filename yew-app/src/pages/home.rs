use crate::components::game_button;
use crate::router;
use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <>
            <div>
                <game_button::GameButton text={"Local-Multiplayer"} route={router::Route::LocalMultiplayer} />
            </div>
            <div class={"menu-container"}>
                <p>{"Menu"}</p>
                <btn class="menu-btn">{"test-btn"}</btn>
            </div>
        </>
    }
}
