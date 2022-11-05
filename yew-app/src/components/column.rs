use crate::constants::*;
use crate::util::board_state::BoardState;
use crate::util::util::{DiskColor, DiskData};
use yew::{classes, html, Callback, Component, Context, Html, MouseEvent, Properties};

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Properties, PartialEq)]
pub struct ColumnProperties {
    pub col_num: usize,
    pub disks: Rc<RefCell<BoardState>>,
    pub in_game: bool,
    pub rerender_board_callback: Callback<MouseEvent>,
}

pub enum ColumnMessages {
    Rerender(MouseEvent),
    NoChange,
}

pub struct Column;

impl Component for Column {
    type Message = ColumnMessages;
    type Properties = ColumnProperties;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ColumnMessages::Rerender(event) => {
                if ctx.props().disks.borrow().game_won {
                    ctx.props().rerender_board_callback.emit(event);
                }
                true
            }
            _ => false,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                {if ctx.props().in_game && !ctx.props().disks.borrow().game_won {html!{<button
                    class={ "btn" }
                    style={format!("grid-column-start: {}", ctx.props().col_num + 1)}
                    onclick={ self.create_onclick(ctx) }
                />}} else {html!{}}}
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
        ctx.link().callback(move |event| {
            let disks = &mut board.borrow_mut();
            if disks.game_won {
                return ColumnMessages::NoChange;
            }
            for i in (0..BOARD_HEIGHT).rev() {
                if disks.board_state[i][col_num] == DiskColor::Empty {
                    if disks.check_winner(DiskData::new(i, col_num, disks.current_player)) {
                        disks.game_won = true;
                    }
                    (disks.board_state[i][col_num], disks.current_player) =
                        if disks.current_player == DiskColor::P1 {
                            (DiskColor::P1, DiskColor::P2)
                        } else {
                            (DiskColor::P2, DiskColor::P1)
                        };
                    let num_moves = disks.num_moves;
                    disks.game_history[num_moves] = col_num;
                    disks.num_moves += 1;
                    if disks.num_moves == BOARD_WIDTH * BOARD_HEIGHT {
                        disks.game_won = true;
                    }
                    return ColumnMessages::Rerender(event);
                }
            }
            ColumnMessages::NoChange
        })
    }
}
