use crate::components::game_button;
use crate::router;
use yew::prelude::*;

pub struct Home;

impl Component for Home {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        //let history = use_history().unwrap();
        html! {
            <>
                <div>
                    <game_button::GameButton text={"Local-Multiplayer"} route={router::Route::LocalMultiplayer} />
                </div>
            </>
        }
    }
}
