use crate::yew::{Children, Callback, Component, Html, html, Context, Properties};
use crate::Router;

struct Board;

impl Component for Board {

    type Message = ();
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onmouseover = {
            Callback::from(|_| ())
        };
        let onmouseout = {

        };
        let onclick = {
            Callback::from(|mouse_event| {

            })
        };
        html! {
            <>
                {(0..=6).iter().map(|num| {
                    html! {<button key=num onclick />}
                }).collect::<Html>()}
                <Switch<Router::Route> render={Switch::render(switch_route)} />
            <>
        }
    }

}