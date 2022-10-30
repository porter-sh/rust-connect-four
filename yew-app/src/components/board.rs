use yew::{Children, Callback, Component, Html, html, Context, Properties};
use yew_router::prelude::*;

#[derive(Clone)]
pub struct Board {
    board: [[Disk; 6]; 7]
}

#[derive(Clone, Copy)]
pub enum Disk {
    Empty,
    P1,
    P2
}

impl Component for Board {

    type Message = ();
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {board: [[Disk::Empty; 6]; 7]}
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