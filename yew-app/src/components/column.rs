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
use yew::{classes, html, Callback, Component, Context, Html, KeyboardEvent, MouseEvent, Properties};
use gloo::events::EventListener;
use wasm_bindgen::JsCast;

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
    Rerender,
    NoChange,
}

/// Column component to represent a given column of the Board
/// When clicked, drops a disk into the Column if it is not full
/// When not in a game or if the game is won, Column will not accept player input
pub struct Column {
    onclick: Callback<MouseEvent>, // Callback to drop a disk into the Column
    global_keyboard_listener: RefCell<Option<EventListener>>
}

/// Allows Column to be used as an HTML component
impl Component for Column {
    type Message = ColumnMessages;
    type Properties = ColumnProperties;

    /// Creates the Column component and creates the onclick callback
    fn create(ctx: &Context<Self>) -> Self {
        let col_num = ctx.props().col_num as u8;
        let onclick = {
            let board = Rc::clone(&ctx.props().disks);
            // let rerender_board_callback = ctx.props().rerender_board_callback.clone();
            ctx.link().callback(move |_| {
                let mut disks = board.borrow_mut();
                if !disks.board_state.is_col_full(col_num) {
                    disks.make_move(col_num).unwrap();
                    disks.update_server_if_online(col_num);
                    disks.run_ai_if_applicable();

                    return ColumnMessages::Rerender;
                }

                ColumnMessages::NoChange
            })
        };
        Self {
            onclick,
            global_keyboard_listener: RefCell::new(None)
        }
    }

    /// Rerenders the Column if msg == Rerender
    /// If the game is won, the Board will also be rerendered, so all Columns update to not accept user input
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ColumnMessages::Rerender => {
                let disks = ctx.props().disks.borrow();
                if !disks.can_move
                    || disks.second_player_extension.is_ai()
                    || disks.second_player_extension.is_survival_mode()
                {
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
                {if ctx.props().in_game && ctx.props().disks.borrow().can_move
                        && !ctx.props().disks.borrow().board_state.is_col_full(ctx.props().col_num) {
                    let onclick = self.onclick.clone();
                    let col_num = ctx.props().col_num;
                    if self.global_keyboard_listener.borrow().is_none() {
                        *self.global_keyboard_listener.borrow_mut() = Some(EventListener::new(
                            &gloo::utils::document(),
                            "keydown",
                            move |event| {
                                if let Some(key_event) = event.dyn_ref::<KeyboardEvent>() {
                                    if key_event.key() == (col_num + 1).to_string() {
                                        onclick.emit(MouseEvent::new("mousedown").unwrap());
                                    }
                                }
                            }
                        ));
                    }
                    html!{<button
                        class={ "btn" }
                        style={format!("grid-column-start: {}", ctx.props().col_num + 1)}
                        onclick={ self.onclick.clone() }
                    />}
                } else {
                    *self.global_keyboard_listener.borrow_mut() = None;
                    html!{}
                }}
                {(0..(BOARD_HEIGHT as u8)).into_iter().map(|row_num| html! { // Display all disks in the Column
                    <div
                        class={classes!(ctx.props().style_of_disk(row_num))}
                        style={format!("grid-column-start: {}; grid-row-start: {};", ctx.props().col_num + 1, BOARD_HEIGHT - row_num)}
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
