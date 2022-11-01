use crate::components::column::*;
use crate::constants::*;
use yew::{html, /*Callback, Children,*/ Component, Context, Html /*Properties*/};
// use yew_router::prelude::*;
use gloo::console::log;

use std::cell::RefCell;

pub struct Board {
    board: RefCell<Disks>,
}

pub type Disks = [[Disk; BOARD_WIDTH]; BOARD_HEIGHT];

#[derive(Clone, Copy, PartialEq)]
pub enum Disk {
    Empty,
    P1,
    P2,
}

impl Component for Board {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        log!("NEW BOARD");
        Self {
            board: RefCell::new([[Disk::Empty; BOARD_WIDTH]; BOARD_HEIGHT]),
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class={"board-background"}>
                {(0..BOARD_WIDTH).into_iter().map(|num| {
                    html! {
                        <Column col_num={ num } disks={ RefCell::clone(&self.board) } />
                    }
                }).collect::<Html>()}
            </div>
        }
    }
}
