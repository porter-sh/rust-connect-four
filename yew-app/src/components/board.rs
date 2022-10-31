use yew::{Children, Callback, Component, Html, html, Context, Properties};
use yew_router::prelude::*;
use crate::components::column::*;

use std::cell::RefCell;
use std::sync::{Mutex, Arc};

pub struct Board {
    board: RefCell<[[Disk; 6]; 7]>
}

#[derive(Clone, Copy, PartialEq)]
pub enum Disk {
    Empty,
    P1,
    P2
}

impl Component for Board {

    type Message = ();
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {board: RefCell::new([[Disk::Empty; 6]; 7])}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                {(0..7).into_iter().map(|num| {
                    html! {
                        <Column col_num={ num } disks={ RefCell::clone(&self.board) } />
                    }
                }).collect::<Html>()}
            </div>
        }
    }
}