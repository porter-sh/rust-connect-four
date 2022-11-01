use crate::components::column::*;
use crate::constants::*;
use yew::{html, /*Callback, Children,*/ Component, Context, Html /*Properties*/};
// use yew_router::prelude::*;
use gloo::console::log;

use std::rc::Rc;
use std::cell::RefCell;

pub struct Board {
    board: Rc<RefCell<BoardState>>,
}

#[derive(PartialEq)]
pub struct BoardState {
    pub board_state: Disks,
    pub current_player: Disk
}

impl Default for BoardState {
    fn default() -> Self {
        Self {
            board_state: [[Disk::Empty; BOARD_WIDTH]; BOARD_HEIGHT],
            current_player: Disk::P1
        }
    }
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
            board: Rc::new(RefCell::new(Default::default())),
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class={"board-background"}>
                {(0..BOARD_WIDTH).into_iter().map(|num| {
                    html! {
                        <Column col_num={ num } disks={ Rc::clone(&self.board) } />
                    }
                }).collect::<Html>()}
            </div>
        }
    }
}
