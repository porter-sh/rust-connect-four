//! Contains definition of GameControlButtons.
//! GameControlButtons are the buttons underneath the game board that allow
//! for user input outside of the game, like "Quit Game" and "Undo".
//! All of the buttons are created in this file, to make it easy to have
//! them all within the same <div> element.
use crate::{
    router::Route,
    util::{board_state::BoardState, util::DiskColor},
};
use std::{cell::RefCell, rc::Rc};
use yew::{classes, html, Callback, Component, Context, Html, MouseEvent, Properties};
use yew_router::prelude::*;

use gloo::console::{error, log};

use super::board::BoardMessages;

#[derive(Properties, PartialEq)]
pub struct UtilityBarProperties {
    pub board: Rc<RefCell<BoardState>>, // Mutably share BoardState across components
    pub rerender_board_callback: Callback<BoardMessages>, // Tells the Board component to rerender
}

pub struct UtilityBar {
    undo_callback: Callback<MouseEvent>,
    info_string: String,
}

impl Component for UtilityBar {
    type Message = ();
    type Properties = UtilityBarProperties;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            undo_callback: Self::create_undo_move_callback(
                Rc::clone(&ctx.props().board),
                ctx.props().rerender_board_callback.clone(),
            ),
            info_string: "TODO delete".to_string(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if let Some(history) = ctx.link().history() {
            if let Some(route) = history.location().route::<Route>() {
                html! {
                    <div class="utility-container">
                        <span class={classes!("utility-left")}>
                            {match route {
                                Route::LocalMultiplayer | Route::VersusBot | Route::OnlineMultiplayer => html! {
                                    <button class="utility-btn"
                                        onclick={
                                            Callback::from(move |_|  history.push(Route::Home)) // route home when clicked
                                        }> { "Quit Game" }
                                    </button> },
                                _ => html! {},
                            }}

                            {match route {
                                Route::LocalMultiplayer | Route::VersusBot => html! {
                                    <button class="utility-btn" onclick={self.undo_callback.clone()}>
                                        { "Undo" }
                                    </button> },
                                Route::OnlineMultiplayer => {
                                    let disks = ctx.props().board.borrow();
                                    if !disks.can_move
                                        && disks.second_player_extension.undo_enabled_for_online()
                                        && disks.num_moves > 0
                                    {
                                        html! {
                                            <button class="utility-btn" onclick={self.undo_callback.clone()}>
                                                { "Undo" }
                                            </button>
                                        }
                                    } else {
                                        html! {}
                                    }
                                }
                                _ => html! {},
                            }}
                        </span>

                        <span class={classes!("utility-right", "utility-text-p1")}>
                            {match route {
                                Route::Home | Route::LobbySelect
                                        | Route::AISelect | Route::NotFound
                                        | Route::NotFoundNeedsRedirect => {""},
                                Route::LocalMultiplayer => {
                                    log!("LocalMultiplayer");
                                    if ctx.props().board.borrow().current_player == DiskColor::P1 {
                                        "Player 1's Turn."
                                    } else { "Player 2's Turn." }
                                },
                                Route::VersusBot => {
                                    if ctx.props().board.borrow().current_player == DiskColor::P1 {
                                        "Your Turn."
                                    } else { "Calculating..." }
                                },
                                Route::OnlineMultiplayer => {
                                    if ctx.props().board.borrow().current_player == DiskColor::P1 {
                                        "Your Turn."
                                    } else { "Opponent's Turn." }
                                },
                            }}
                        </span>
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

impl UtilityBar {
    fn create_undo_move_callback(
        board: Rc<RefCell<BoardState>>,
        rerender_board_callback: Callback<BoardMessages>,
    ) -> Callback<MouseEvent> {
        Callback::from(move |_| {
            board.borrow_mut().undo_move_and_handoff_to_second_player();

            // Tell the Board to rerender
            rerender_board_callback.emit(BoardMessages::Rerender);
        })
    }
}
