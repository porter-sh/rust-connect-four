//! column contains the Column component
//!
//! Column components are used by the Board component to display the BoardState,
//! to register when a disk is dropped into the given column of the board,
//! and to accordingly update and display state
//!
//! When not in a game or if the game is won, Column will not accept player input

use crate::util::{board_state::BoardState, util::DiskColor};
use constants::*;
use std::{cell::RefCell, rc::Rc};
use yew::{classes, html, Callback, Component, Context, Html, MouseEvent, Properties};

/// Properties to allow the UndoButton to interact with other components
#[derive(Properties, PartialEq)]
pub struct ColumnProperties {
    pub col_num: u8,                           // Which column of the Board this Column is
    pub disks: Rc<RefCell<BoardState>>,        // Mutably share BoardState across components
    pub in_game: bool, // Whether Column should allow players to click it and drop disks
    pub rerender_board_callback: Callback<()>, // Tells the Board component to rerender
}

/// A message enum to tell the Column whether to rerender or not
pub enum ColumnMessages {
    Rerender, // MouseEvent type is irrelevant, supplied to rerender_board_callback as a necessary, but unused, parameter
    NoChange,
}

/// Column component to represent a given column of the Board
/// When clicked, drops a disk into the Column if it is not full
/// When not in a game or if the game is won, Column will not accept player input
pub struct Column {
    onclick: Callback<MouseEvent>, // Callback to drop a disk into the Column
}

/// Allows Column to be used as an HTML component
impl Component for Column {
    type Message = ColumnMessages;
    type Properties = ColumnProperties;

    /// Creates the Column component and creates the onclick callback
    fn create(ctx: &Context<Self>) -> Self {
        Self {
            onclick: {
                let board = Rc::clone(&ctx.props().disks);
                let col_num = ctx.props().col_num as u8;
                ctx.link().callback(move |_| {
                    let disks = &mut board.borrow_mut();

                    if !disks.board_state.is_col_full(col_num) {
                        disks.make_move(col_num);
                        disks.update_server_if_online(col_num);
                        disks.run_ai_if_applicable();

                        return ColumnMessages::Rerender;
                    }

                    ColumnMessages::NoChange
                })
            },
        }
    }

    /// Rerenders the Column if msg == Rerender
    /// If the game is won, the Board will also be rerendered, so all Columns update to not accept user input
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ColumnMessages::Rerender => {
                let disks = ctx.props().disks.borrow();
                if !disks.can_move || disks.second_player_extension.is_ai() {
                    // Tell the Board to rerender
                    ctx.props().rerender_board_callback.emit(());
                }
                true
            }
            _ => false,
        }
    }

    /// Renders the Column and the related disks in the Board
    /// If in the middle of a game, allows for user input
    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                {if ctx.props().in_game && ctx.props().disks.borrow().can_move && !ctx.props().disks.borrow().board_state.is_col_full(ctx.props().col_num) {html!{<button
                    class={ "btn" }
                    style={format!("grid-column-start: {}", ctx.props().col_num + 1)}
                    onclick={ self.onclick.clone() }
                />}} else {html!{}}}
                {(0..(BOARD_HEIGHT as u8)).into_iter().map(|row_num| html! { // Display all disks in the Column
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
    fn style_of_disk(&self, row: u8) -> String {
        match self.disks.borrow().board_state.get_disk(row, self.col_num) {
            DiskColor::Empty => "disk-empty",
            DiskColor::P1 => "disk-p1",
            DiskColor::P2 => "disk-p2",
        }
        .to_string()
    }
}
