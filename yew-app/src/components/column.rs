use crate::components::board::{BoardState, Disk};
use crate::constants::*;
use yew::{Callback, classes, html, Component, Context, Html, MouseEvent, Properties};

use std::rc::Rc;
use std::cell::RefCell;

#[derive(Properties, PartialEq)]
pub struct ColumnProperties {
    pub col_num: usize,
    pub disks: Rc<RefCell<BoardState>>,
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
        html! {
            <>
                <button
                    class={"btn"}
                    style={format!("grid-column-start: {}", ctx.props().col_num + 1)}
                    onclick={self.create_onclick(ctx)}
                />
                {(0..BOARD_HEIGHT).into_iter().map(|row_num| html! {
                    <div
                        class={classes!(ctx.props().style_of_disk(row_num))}
                        style={format!("grid-column-start: {}; grid-row-start: {};", ctx.props().col_num + 1, row_num + 1)}
                    />
                }).collect::<Html>()}
            </>
        }
    }
}

impl ColumnProperties {

    fn style_of_disk(&self, row: usize) -> String {
        match self.disks.borrow().board_state[row][self.col_num] {
            Disk::Empty => "disk-empty",
            Disk::P1 => "disk-p1",
            Disk::P2 => "disk-p2",
        }.to_string()
    }

}

impl Column {

    fn create_onclick(&self, ctx: &Context<Self>) -> Callback<MouseEvent> {
        let board = Rc::clone(&ctx.props().disks);
        let col_num = ctx.props().col_num;
        ctx.link().callback(move |_| {
            let disks = &mut board.borrow_mut();
            let mut i = BOARD_HEIGHT - 1;
            loop {
                if disks.board_state[i][col_num] == Disk::Empty {
                    (disks.board_state[i][col_num], disks.current_player) = if disks.current_player == Disk::P1 {
                        (Disk::P1, Disk::P2)
                    } else {
                        (Disk::P2, Disk::P1)
                    };
                    return ColumnMessages::DiskDropped;
                }
                if i == 0 {
                    break;
                } else {
                    i -= 1;
                }
            }
            ColumnMessages::NoChange
        })
    }

}
