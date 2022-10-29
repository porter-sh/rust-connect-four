use yew::{Children, Callback, Component, Html, html, Context, Properties};
use yew_router::prelude::*;
use crate::router;

pub struct Board;

impl Component for Board {

    type Message = ();
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        /*let onmouseover = {
            Callback::from(|_| ())
        };
        let onmouseout = {

        };*/
        html! {
            <>
                {(0..=6).into_iter().map(|num| {
                    html! {<button key={num} onclick={Callback::from(|mouse_event| {})} />}
                }).collect::<Html>()}
            </>
        }
    }

}