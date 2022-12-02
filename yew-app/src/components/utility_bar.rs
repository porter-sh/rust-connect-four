//! Contains definition of UtilityBar.
//! UtilityBar are the buttons underneath the game board that allow
//! for user input outside of the game, like "Quit Game" and "Undo".
//! All of the buttons are created in this file, to make it easy to have
//! them all within the same <div> element.
use super::board::BoardMessages;
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

#[derive(PartialEq)]
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

#[derive(Properties, PartialEq)]
pub struct UtilityBarProperties {
    pub board: Rc<RefCell<BoardState>>, // Mutably share BoardState across components
    pub rerender_board_callback: Callback<BoardMessages>, // Tells the Board component to rerender
}

pub struct UtilityBar {
    undo_callback: Callback<MouseEvent>,
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
                                    if (!disks.can_move != (disks.disks.check_last_drop_won() && disks.disks.get_is_p1_turn() == (disks.current_player == DiskColor::P1)))
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

                        {{
                            let (color_class, message) = Self::get_color_and_message_str(ctx.props().board.borrow());
                            html! {
                                <span class={classes!("utility-right", color_class)}>
                                    { message }
                                </span>
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
                    AI(_) | SurvivalMode(_) => "Your turn.",
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
                    AI(_) | SurvivalMode(_) => "AI's turn.",
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
                    AI(_) | SurvivalMode(_) => "You win!",
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
                    AI(_) | SurvivalMode(_) => "AI wins!",
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
