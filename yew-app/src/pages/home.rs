use yew::prelude::*;
use crate::components;

pub struct Home;
impl Component for Home {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html!{
            <>
                <div>
                    <button>{"Back"}</button>
                </div>
                <components::board::Board />
            </>
        }
    }
}