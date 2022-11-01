use crate::components;
use crate::router;
use yew::prelude::*;
use yew_router::prelude::use_history;
use yew_router::prelude::History;

#[function_component(MyButton)]
fn my_button() -> Html {
    let history = use_history().unwrap();
    html! {
        <button onclick={
            Callback::from(move |_| history.push(router::Route::LocalMultiplayer))
        }>{ "Back" }</button>
    }
}

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
                    <MyButton />
                </div>
                <components::board::Board />
            </>
        }
    }
}
