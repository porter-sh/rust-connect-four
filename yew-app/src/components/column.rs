use crate::components::board::{Disk, Disks};
use crate::constants::*;
use yew::{classes, html, Callback, /*Children,*/ Component, Context, Html, Properties};
// use yew_router::prelude::*;
use gloo::console::log;

use std::cell::RefCell;

#[derive(Properties, PartialEq)]
pub struct ColumnProperties {
    pub col_num: usize,
    pub disks: RefCell<Disks>,
}

pub struct Column;

impl Component for Column {
    type Message = ();
    type Properties = ColumnProperties;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <button class={"btn"} style={format!("grid-column-start: {}", ctx.props().col_num + 1)} onclick={
                    let disks = RefCell::clone(&ctx.props().disks);
                    let col_num = ctx.props().col_num;
                    Callback::from(move |_| {
                        let mut disks = disks.borrow_mut();
                        let i = 0;
                        while i < disks.len() {
                            if disks[i][col_num] == Disk::Empty {
                                disks[i][col_num] = Disk::P1;
                                log!("Dropped disk in row {}, col {}.", i, col_num);
                                return;
                            }
                        }
                    })
                }></button>
                {(0..BOARD_HEIGHT).into_iter().map(|row_num| html! {
                    <div class={classes!(ctx.props().style_of_disk(row_num))} style={format!("grid-column-start: {}; grid-row-start: {};", ctx.props().col_num + 1, row_num + 1)} />
                }).collect::<Html>()}
            </>
        }
    }
}

impl ColumnProperties {
    fn style_of_disk(&self, row: usize) -> String {
        match self.disks.borrow()[row][self.col_num] {
            Disk::Empty => "disk-empty",
            Disk::P1 => "disk-p1",
            Disk::P2 => "disk-p2",
        }
        .to_string()
    }
}
