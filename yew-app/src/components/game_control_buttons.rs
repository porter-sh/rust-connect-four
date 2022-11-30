//! Contains definition of GameControlButtons.
//! GameControlButtons are the buttons underneath the game board that allow
//! for user input outside of the game, like "Quit Game" and "Undo".
//! All of the buttons are created in this file, to make it easy to have
//! them all within the same <div> element.
use crate::{router::Route, util::board_state::BoardState};
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
                            Route::OnlineMultiplayer => {
                                let disks = ctx.props().board.borrow();
                                if !disks.can_move
                                    && disks.second_player_extension.undo_enabled_for_online()
                                    && disks.num_moves > 0
                                {
                                    html! {
                                        <button class="control-btn" onclick={self.undo_callback.clone()}>
                                            { "Undo" }
                                        </button>
                                    }
                                } else {
                                    html! {}
                                }
                            }
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

impl GameControlButtons {
    fn create_undo_move_callback(
        board: Rc<RefCell<BoardState>>,
        rerender_board_callback: Callback<()>,
    ) -> Callback<MouseEvent> {
        Callback::from(move |_| {
            board.borrow_mut().undo_move_and_handoff_to_second_player();

            // Tell the Board to rerender
            rerender_board_callback.emit(());
        })
    }
}
