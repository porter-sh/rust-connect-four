use yew::{Children, Callback, Component, function_component, Html, html, Context, Properties, use_state};
use yew_router::scope_ext::HistoryHandle;
use yew_router::prelude::*;
use crate::router;

/*#[function_component(BackButton)]
fn back_button() -> Html {
    let button_hidden = use_state(|| true);
    html! {
        <button hidden={*button_hidden} />
    }
}*/

struct BackButton {
    hidden: bool,
    route_listener: HistoryHandle
}

impl Component for BackButton {

    type Message = bool;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let route_listener = ctx.link().add_history_listener(
            ctx.link().callback(|msg| false)
        ).unwrap();
        Self {hidden: false, route_listener}
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        // let route = ctx.link().route::<router::Route>();
        self.hidden = msg;
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html!{<button hidden={self.hidden} />}
    }

}