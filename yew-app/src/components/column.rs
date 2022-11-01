use crate::components::board_state::{BoardState, DiskColor, DiskData};
use crate::constants::*;
use gloo::console::log;
use yew::{classes, html, Callback, Component, Context, Html, MouseEvent, Properties};
use yew_router::prelude::*;
use yew_router::scope_ext::HistoryHandle;

use std::cell::{Ref, RefCell};
use std::rc::Rc;

#[derive(Properties, PartialEq)]
pub struct ColumnProperties {
    pub col_num: usize,
    pub disks: Rc<RefCell<BoardState>>,
    pub in_game: bool,
}

pub enum ColumnMessages {
    Rerender,
    NoChange,
}

pub struct Column;

impl Component for Column {
    type Message = ColumnMessages;
    type Properties = ColumnProperties;

    fn create(ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ColumnMessages::Rerender => true,
            _ => false,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <button
                    class={"btn"}
                    style={format!("grid-column-start: {}", ctx.props().col_num + 1)}
                    onclick={
                        if ctx.props().in_game {
                            self.create_onclick(ctx)
                        } else {
                            ctx.link().callback(|_| ColumnMessages::NoChange)
                        }
                    }
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
            DiskColor::Empty => "disk-empty",
            DiskColor::P1 => "disk-p1",
            DiskColor::P2 => "disk-p2",
        }
        .to_string()
    }
}

impl Column {
    fn create_onclick(&self, ctx: &Context<Self>) -> Callback<MouseEvent> {
        let board = Rc::clone(&ctx.props().disks);
        let col_num = ctx.props().col_num;
        ctx.link().callback(move |_| {
            let disks = &mut board.borrow_mut();
            if disks.game_won {
                return ColumnMessages::NoChange;
            }
            let mut i = BOARD_HEIGHT - 1;
            loop {
                if disks.board_state[i][col_num] == DiskColor::Empty {
                    log!(disks.check_winner(DiskData::new(i, col_num, disks.current_player)));
                    (disks.board_state[i][col_num], disks.current_player) =
                        if disks.current_player == DiskColor::P1 {
                            (DiskColor::P1, DiskColor::P2)
                        } else {
                            (DiskColor::P2, DiskColor::P1)
                        };
                    return ColumnMessages::Rerender;
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
