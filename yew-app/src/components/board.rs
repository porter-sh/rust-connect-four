//! board contains the Board component
//! Board contains the internal BoardState, and renders that state through Column components
//! Board also accepts user input when in the middle of a game via Column components

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

use super::{
    column::*,
    utility_bar::{InfoMessage, UtilityBar},
};
use crate::{
    router::{AIRoute, Route},
    util::{
        board_state::BoardState,
        util::{GameUpdateMessage, SecondPlayerAIMode, SecondPlayerSurvivalAIMode},
    },
};
use constants::*;
use std::{cell::RefCell, rc::Rc};
use yew::{html, Component, Context, Html};
use yew_router::prelude::*;

pub enum BoardMessages {
    Rerender,
    RerenderUtilityBar,
    RerenderAndUpdateBoard(GameUpdateMessage),
}

/// Board component to store state of the board, to render the board, and to accept user input
pub struct Board {
    board: Rc<RefCell<BoardState>>, // Mutably share BoardState across components
    _location_handle: LocationHandle, // when not dropped allows the Board to respond to route changes
}

impl Component for Board {
    type Message = BoardMessages;
    type Properties = ();

    /// Creates the Board component and adds a history listener to selectively react to and rerender on route changes
    fn create(ctx: &Context<Self>) -> Self {
        Board::new(ctx)
    }

    /// Rerender when a message is recieved
    /// All messages sent will be to request a rerender of the entire Board
    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        // log!("Updating Board");
        if let BoardMessages::RerenderAndUpdateBoard(msg) = msg {
            self.board
                .borrow_mut()
                .update_state_from_second_player_message(msg);
        }
        true
    }

    /// Renders the Board
    /// If in the middle of a game, allows for user input
    /// Renders an UndoButton if playing a supported gamemode
    fn view(&self, ctx: &Context<Self>) -> Html {
        let rerender_board_callback = ctx.link().callback(|msg: BoardMessages| msg);
        let route = ctx.link().route::<Route>().unwrap_or(Route::Home);

        html! {
            <>
                <div class={ "board-background" }>
                    {(0..(BOARD_WIDTH as u8)).into_iter().map(|num| { // Create Columns for the Board
                        html! {
                            <Column col_num={ num } disks={ Rc::clone(&self.board) } in_game={ // Accept input if in game
                                match route {
                                    Route::LocalMultiplayer
                                        | Route::VersusBot
                                        | Route::OnlineMultiplayer => {
                                            true
                                        },
                                    _ => false
                                }
                            } rerender_board_callback={ rerender_board_callback.clone() } />
                        }
                    }).collect::<Html>()}
                </div>
                <UtilityBar board={ Rc::clone(&self.board) }
                    rerender_board_callback={ rerender_board_callback.clone() } />
            </>
        }
    }
}

impl Board {
    pub fn new(ctx: &Context<Board>) -> Self {
        let board_origin = Rc::new(RefCell::new(BoardState::new(
            ctx.link()
                .callback(|msg: GameUpdateMessage| BoardMessages::RerenderAndUpdateBoard(msg)),
        )));
        Self {
            board: Rc::clone(&board_origin),
            _location_handle: Self::get_location_handle(ctx, board_origin),
        }
    }

    fn get_location_handle(ctx: &Context<Board>, board: Rc<RefCell<BoardState>>) -> LocationHandle {
        ctx.link()
            .add_location_listener(ctx.link().callback(move |location: Location| {
                let board_clone = Rc::clone(&board);
                // Will rerender the Board
                Self::on_reroute(board_clone, location);
                BoardMessages::Rerender
            }))
            .unwrap()
    }

    fn on_reroute(board: Rc<RefCell<BoardState>>, location: Location) {
        let path = location.path();
        gloo::console::log!(format!("/local-multiplayer: {:?},\n /local-multiplayer/: {:?},\n /rust-connect-four/local-multiplayer: {:?},\n /rust-connect-four/local-multiplayer/: {:?},\n rust-connect-four/local-multiplayer: {:?},\n rust-connect-four/local-multiplayer/: {:?}",
            Route::recognize("/local-multiplayer").unwrap(),
            Route::recognize("/local-multiplayer/").unwrap(),
            Route::recognize("/rust-connect-four/local-multiplayer").unwrap(),
            Route::recognize("/rust-connect-four/local-multiplayer/").unwrap(),
            Route::recognize("rust-connect-four/local-multiplayer").unwrap(),
            Route::recognize("rust-connect-four/local-multiplayer/").unwrap()));
        gloo::console::log!(format!("Path: {}", path));
        let new_path = path.strip_prefix("/rust-connect-four").unwrap_or_default();
        gloo::console::log!(format!("new_path: {}", new_path));
        gloo::console::log!(format!(
            "new_path recognized: {:?}",
            Route::recognize(new_path)
        ));
        if let Some(route) = Route::recognize(path) {
            gloo::console::log!(format!("{:?}", route));
            match route {
                Route::LocalMultiplayer => {
                    board.borrow_mut().reset(); // Reset the BoardState when starting a new game
                }
                Route::OnlineMultiplayer => {
                    // let query_string = location.search();
                    // let lobby = query_string.split("=").collect::<Vec<&str>>()[1];
                    // board.borrow_mut().init_online(lobby.to_string());
                    // let query = location.query();
                }
                Route::VersusBot => {
                    if let Some(ai_route) = AIRoute::recognize(location.path()) {
                        match ai_route {
                            AIRoute::Random => {
                                board.borrow_mut().init_ai(SecondPlayerAIMode::Random)
                            }
                            AIRoute::BruteForce => {
                                board.borrow_mut().init_ai(SecondPlayerAIMode::Perfect)
                            }
                            AIRoute::Survival => board
                                .borrow_mut()
                                .init_survival(SecondPlayerSurvivalAIMode::Perfect),
                        };
                    }
                }
                _ => {
                    let mut board = board.borrow_mut();
                    board.second_player_extension.remove_extension();
                    board.info_message = InfoMessage::NoMessage;
                }
            }
        }
    }
}
