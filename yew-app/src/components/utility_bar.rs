//! Contains definition of UtilityBar.
//! UtilityBar are the buttons underneath the game board that allow
//! for user input outside of the game, like "Quit Game" and "Undo".
//! All of the buttons are created in this file, to make it easy to have
//! them all within the same <div> element.

/*
 * This file is part of Rust-Connect-Four
 * Copyright (C) 2022 Alexander Broihier <alexanderbroihier@gmail.com>
 * Copyright (C) 2022 Porter Shawver <portershawver@gmail.com>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published
 * by the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

use super::board::BoardMessage;
use crate::{
    router::Route,
    util::{
        board_state::BoardState,
        util::{DiskColor, SecondPlayerExtensionMode},
    },
};
use std::{
    cell::{Ref, RefCell},
    rc::Rc,
};
use yew::{classes, html, Callback, Component, Context, Html, MouseEvent, Properties};
use yew_router::prelude::*;
use SecondPlayerExtensionMode::{None, OnlinePlayer, SurvivalMode, AI};

use gloo::console::error;

/// Message to display to the player
#[derive(PartialEq, Debug)]
pub enum InfoMessage {
    P1Turn,
    P2Turn,
    P1Win,
    P2Win,
    Draw,
    WaitingForOpponent,
    Connecting,
    ConnectionFailed,
    NoMessage,
}

/// Properties to allow the UtilityBar to interact with the board
#[derive(Properties, PartialEq)]
pub struct UtilityBarProperties {
    pub board: Rc<RefCell<BoardState>>, // Mutably share BoardState across components
    pub rerender_board_callback: Callback<BoardMessage>, // Tells the Board component to rerender
}

/// UtilityBar component to allow players to quit games and undo moves
/// Also displays relevant information to the player
pub struct UtilityBar {
    undo_callback: Callback<MouseEvent>,
}

impl Component for UtilityBar {
    type Message = ();
    type Properties = UtilityBarProperties;

    /// Creates the UtilityBar component and the undo_callback
    fn create(ctx: &Context<Self>) -> Self {
        Self {
            undo_callback: Self::create_undo_move_callback(
                Rc::clone(&ctx.props().board),
                ctx.props().rerender_board_callback.clone(),
            ),
        }
    }

    /// Renders the UtilityBar
    /// If in a game, provides a button to quit the game
    /// If in a game where undo is allowed, provides a button to undo moves
    fn view(&self, ctx: &Context<Self>) -> Html {
        if let Some(navigator) = ctx.link().navigator() {
            if let Some(route) = ctx.link().route::<Route>() {
                html! {
                    <div class="utility-container">
                        <span class={classes!("utility-left")}>
                            {match route { // Render the Quit Game button if applicable
                                Route::LocalMultiplayer | Route::VersusBot | Route::OnlineMultiplayer => html! {
                                    <button class="utility-btn"
                                        onclick={
                                            Callback::from(move |_|  navigator.push(&Route::Home)) // route home when clicked
                                        }> { "Quit Game" }
                                    </button> },
                                _ => html! {},
                            }}

                            {match route {// Render the Undo button if applicable
                                Route::LocalMultiplayer | Route::VersusBot => html! {
                                    if ctx.props().board.borrow().num_moves != 0 {
                                        <button class="utility-btn" onclick={self.undo_callback.clone()}>
                                            { "Undo" }
                                        </button>
                                    }
                                },
                                Route::OnlineMultiplayer => {
                                    let disks = ctx.props().board.borrow();
                                    if (!disks.can_move
                                        != (disks.disks.check_last_drop_won() && disks.disks.get_is_p1_turn()
                                            == (disks.current_player == DiskColor::P1)
                                        ))
                                        && disks.second_player_extension.undo_enabled_for_online()
                                        && disks.num_moves != 0
                                    { // If the player can currently undo their last move
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

                        {{ // Diplay info text
                            let (color_class, message) = Self::get_color_and_message_str(ctx.props().board.borrow());
                            html! {
                                <>
                                    <span class={classes!("utility-right", color_class)}>
                                        <div class={classes!(color_class)}>{ message }</div>
                                        {{
                                            if let Some(difficulty) = ctx.props().board.borrow().get_survival_mode_difficulty(){
                                                html!{
                                                    <div style={"padding-top:10px"} class={classes!("utility-text-plain")}>
                                                        { format!("Difficulty: {}", difficulty) }
                                                    </div>
                                                }
                                            } else {
                                                html!{}
                                            }
                                        }}
                                    </span>
                                </>
                            }
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

impl UtilityBar {
    // Creates a callback that undoes the last made move
    fn create_undo_move_callback(
        board: Rc<RefCell<BoardState>>,
        rerender_board_callback: Callback<BoardMessage>,
    ) -> Callback<MouseEvent> {
        Callback::from(move |_| {
            // Undo the move
            board.borrow_mut().undo_move_and_handoff_to_second_player();

            // Tell the Board to rerender
            rerender_board_callback.emit(BoardMessage::Rerender);
        })
    }

    // Gets the color and message to display as info text from the current board state
    fn get_color_and_message_str(board: Ref<BoardState>) -> (&'static str, &'static str) {
        match board.info_message {
            InfoMessage::P1Turn => (
                "utility-text-p1",
                match board.get_second_player_mode() {
                    OnlinePlayer { .. } => match board.current_player {
                        DiskColor::P1 => "Your turn",
                        DiskColor::P2 => "Opponent's turn.",
                        DiskColor::Empty => "Red's turn.",
                    },
                    AI { ai_color, .. } | SurvivalMode { ai: _, ai_color } => {
                        if ai_color == &DiskColor::P2 {
                            "Your turn."
                        } else {
                            "AI's turn."
                        }
                    }
                    None => "Red's turn.",
                },
            ),
            InfoMessage::P2Turn => (
                "utility-text-p2",
                match board.get_second_player_mode() {
                    OnlinePlayer { .. } => match board.current_player {
                        DiskColor::P1 => "Opponent's turn.",
                        DiskColor::P2 => "Your turn",
                        DiskColor::Empty => "Yellow's turn.",
                    },
                    AI { ai_color, .. } | SurvivalMode { ai: _, ai_color } => {
                        if ai_color == &DiskColor::P2 {
                            "AI's turn."
                        } else {
                            "Your turn."
                        }
                    }
                    None => "Yellow's turn.",
                },
            ),
            InfoMessage::P1Win => (
                "utility-text-p1",
                match board.get_second_player_mode() {
                    OnlinePlayer { .. } => match board.current_player {
                        DiskColor::P1 => "You win!",
                        DiskColor::P2 => "You lose.",
                        DiskColor::Empty => "Red wins.",
                    },
                    AI { ai_color, .. } | SurvivalMode { ai: _, ai_color } => {
                        if ai_color == &DiskColor::P2 {
                            "You win!"
                        } else {
                            "AI wins."
                        }
                    }
                    None => "Red wins!",
                },
            ),
            InfoMessage::P2Win => (
                "utility-text-p2",
                match board.get_second_player_mode() {
                    OnlinePlayer { .. } => match board.current_player {
                        DiskColor::P1 => "You lose.",
                        DiskColor::P2 => "You win!",
                        DiskColor::Empty => "Red wins.",
                    },
                    AI { ai_color, .. } | SurvivalMode { ai: _, ai_color } => {
                        if ai_color == &DiskColor::P2 {
                            "AI wins."
                        } else {
                            "You win!"
                        }
                    }
                    None => "Yellow wins!",
                },
            ),
            InfoMessage::Draw => ("utility-text-plain", "Draw."),
            InfoMessage::WaitingForOpponent => ("utility-text-plain", "Waiting for opponent..."),
            InfoMessage::Connecting => ("utility-text-plain", "Connecting..."),
            InfoMessage::ConnectionFailed => ("utility-text-plain", "Connection failed."),
            InfoMessage::NoMessage => ("utility-text-plain", ""),
        }
    }
}
