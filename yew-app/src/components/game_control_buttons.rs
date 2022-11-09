use crate::{
    constants::BOARD_HEIGHT,
    router::Route,
    util::{board_state::BoardState, util::DiskColor},
};
use std::{cell::RefCell, rc::Rc};
use yew::{html, Callback, Component, Context, Html, MouseEvent, Properties};
use yew_router::prelude::*;

use gloo::console::error;

#[derive(Properties, PartialEq)]
pub struct GameControlButtonProperties {
    pub board: Rc<RefCell<BoardState>>, // Mutably share BoardState across components
    pub rerender_board_callback: Callback<()>, // Tells the Board component to rerender
}

pub struct GameControlButtons {
    undo_callback: Callback<MouseEvent>,
}

impl Component for GameControlButtons {
    type Message = ();
    type Properties = GameControlButtonProperties;

    fn create(ctx: &Context<Self>) -> Self {
        let board = Rc::clone(&ctx.props().board);
        let rerender_board_callback_clone = ctx.props().rerender_board_callback.clone();
        Self {
            undo_callback: Callback::from(move |_| {
                // Limit the scope of BoardState mutable borrow so other components can check the BoardState when they rerender
                {
                    let mut disks = board.borrow_mut();
                    if disks.num_moves == 0 {
                        return;
                    } // At the start of the game

                    disks.game_won = false; // Undoes win, allowing board interaction

                    // Revert to previous player
                    disks.current_player = if disks.current_player == DiskColor::P1 {
                        DiskColor::P2
                    } else {
                        DiskColor::P1
                    };

                    disks.num_moves -= 1;

                    let num_moves = disks.num_moves;

                    let col = disks.game_history[num_moves]; // Get the column the last move was made in
                    for row in 0..BOARD_HEIGHT {
                        if disks.board_state[row][col] != DiskColor::Empty {
                            // First nonempty space is the last move in this column
                            disks.board_state[row][col] = DiskColor::Empty;
                            break;
                        }
                    }
                } // Mutable borrow of the BoardState dropped, so other components can check the BoardState when they rerender

                // Tell the Board to rerender (MouseEvent type irrelevant)
                rerender_board_callback_clone.emit(());
            }),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if let Some(history) = ctx.link().history() {
            if let Some(route) = history.location().route::<Route>() {
                html! {
                    <div class="control-container">
                        {match route {
                            Route::LocalMultiplayer | Route::VersusBot | Route::OnlineMultiplayer => html! {
                                <button class="control-btn"
                                    onclick={
                                        Callback::from(move |_|  history.push(Route::Home)) // route home when clicked
                                    }> { "Quit Game" }
                                </button> },
                            _ => html! {},
                        }}

                        {match route {
                            Route::LocalMultiplayer | Route::VersusBot => html! {
                                <button class="control-btn" onclick={self.undo_callback.clone()}>
                                    { "Undo" }
                                </button> },
                                _ => html! {},
                        }}
                    </div>
                }
            } else {
                error!("Error finding page history for game control buttons.");
                html! {}
            }
        } else {
            error!("Error rendering game control buttons.");
            html! {}
        }
    }
}
