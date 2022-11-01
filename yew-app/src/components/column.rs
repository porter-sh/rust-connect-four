use crate::components::board::{Disk, Disks};
use crate::constants::*;
use yew::{classes, html, Callback, /*Children,*/ Component, Context, Html, Properties};
// use yew_router::prelude::*;
use gloo::console::log;
use yew::MouseEvent;

use std::cell::RefCell;

#[derive(Properties, PartialEq)]
pub struct ColumnProperties {
    pub col_num: usize,
    pub disks: RefCell<Disks>,
}

pub enum ColumnMessages {
    DiskDropped,
    NoChange,
}

pub struct Column;

impl Component for Column {
    type Message = ColumnMessages;
    type Properties = ColumnProperties;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        match _msg {
            ColumnMessages::DiskDropped => true,
            _ => false,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        /*let disks = RefCell::clone(&ctx.props().disks);
        let col_num = ctx.props().col_num;
        let onclick = ctx.link().callback(move |_| {
            let mut disks = disks.borrow_mut();
            let mut i = 0;
            log!(disks[0][0] == Disk::Empty);
            while i < BOARD_HEIGHT {
                if disks[i][col_num] == Disk::Empty {
                    disks[i][col_num] = Disk::P1;
                    log!(disks[0][0] == Disk::Empty);
                    log!(format!("Dropped disk in row {}, col {}.", i, col_num));
                    return ColumnMessages::DiskDropped;
                }
                i += 1;
            }
            ColumnMessages::NoChange
        });*/
        let send_message = ctx
            .link()
            .callback(|_: MouseEvent| ColumnMessages::DiskDropped);
        html! {
            <>
                <button class={"btn"} style={format!("grid-column-start: {}", ctx.props().col_num + 1)} onclick={
                    let disks = RefCell::clone(&ctx.props().disks);
                    let col_num = ctx.props().col_num;
                    Callback::from(move |msg| {
                        {
                            let mut disks = disks.borrow_mut();
                            let mut i = 0;
                            log!(disks[0][0] == Disk::Empty);
                            while i < BOARD_HEIGHT {
                                if disks[i][col_num] == Disk::Empty {
                                    disks[i][col_num] = Disk::P1;
                                    log!(disks[0][0] == Disk::Empty);
                                    log!(format!("Dropped disk in row {}, col {}.", i, col_num));
                                    break; //ColumnMessages::DiskDropped;
                                }
                                i += 1;
                            }
                        }
                        // send_message(msg);
                        // ColumnMessages::NoChange
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
