//! board contains the Board component
//! Board contains the internal BoardState, and renders that state through Column components
//! Board also accepts user input when in the middle of a game via Column components

use crate::{
    components::{column::*, game_control_buttons::GameControlButtons},
    constants::*,
    router::Route,
    util::{board_state::BoardState, net, util::DiskColor},
};
use std::{cell::RefCell, rc::Rc};
use yew::{html, Component, Context, Html};
use yew_router::{prelude::*, scope_ext::HistoryHandle};

use gloo::console::log;

pub enum BoardMessages {
    Rerender,
    RerenderAndUpdateColumn(u8),
}

/// Board component to store state of the board, to render the board, and to accept user input
pub struct Board {
    board: Rc<RefCell<BoardState>>, // Mutably share BoardState across components
    #[allow(unused)]
    history_handle: HistoryHandle, // when not dropped allows the Board to respond to route changes
}

impl Component for Board {
    type Message = BoardMessages;
    type Properties = ();

    /// Creates the Board component and adds a history listener to selectively react to and rerender on route changes
    fn create(ctx: &Context<Self>) -> Self {
        let board = Rc::new(RefCell::new(Default::default()));
        let callback = ctx
            .link()
            .callback(|col_num: u8| BoardMessages::RerenderAndUpdateColumn(col_num));
        let history_handle = {
            let board = Rc::clone(&board);
            ctx.link()
                .add_history_listener(ctx.link().callback(move |history: AnyHistory| {
                    // Will rerender the Board
                    if let Some(route) = history.location().route::<Route>() {
                        match route {
                            Route::LocalMultiplayer | Route::VersusBot => {
                                *board.borrow_mut() = Default::default(); // Reset the BoardState when starting a new game
                            }
                            Route::OnlineMultiplayer => {
                                *board.borrow_mut() = BoardState {
                                    socket_writer: match net::spawn_connection_threads(
                                        callback.clone(),
                                    ) {
                                        Ok(writer) => Some(writer),
                                        _ => None,
                                    },
                                    ..Default::default()
                                };
                            }
                            _ => board.borrow_mut().socket_writer = None,
                        }
                    }
                    BoardMessages::Rerender
                }))
                .unwrap() // If an error occured it is likely because this Board is not the child of a router component
        };
        Self {
            board,
            history_handle,
        }
    }

    /// Rerender when a message is recieved
    /// All messages sent will be to request a rerender of the entire Board
    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        if let BoardMessages::RerenderAndUpdateColumn(mut num) = msg {
            let mut board = self.board.borrow_mut();
            if num == ConnectionProtocol::IS_PLAYER_1 {
                board.current_player = DiskColor::P1;
            } else if num == ConnectionProtocol::IS_PLAYER_2 {
                board.current_player = DiskColor::P2;
                board.game_won = true;
            } else if ConnectionProtocol::COL_0 + ConnectionProtocol::WINNING_MOVE_ADDITION <= num
                && num <= ConnectionProtocol::COL_6 + ConnectionProtocol::WINNING_MOVE_ADDITION
            {
                num -= ConnectionProtocol::WINNING_MOVE_ADDITION;
                board.game_won = true;
            } else {
                board.game_won = false;
            }
            if ConnectionProtocol::COL_0 <= num && num <= ConnectionProtocol::COL_6 {
                for row in (0..BOARD_HEIGHT).rev() {
                    if board.board_state[row][num as usize] == DiskColor::Empty {
                        board.board_state[row][num as usize] =
                            if board.current_player == DiskColor::P1 {
                                DiskColor::P2
                            } else {
                                DiskColor::P1
                            };
                        break;
                    }
                }
            }
            log!(format!("Received {}", num));
        }
        true
    }

    /// Renders the Board
    /// If in the middle of a game, allows for user input
    /// Renders an UndoButton if playing a supported gamemode
    fn view(&self, ctx: &Context<Self>) -> Html {
        let rerender_board_callback = ctx.link().callback(|_| BoardMessages::Rerender);
        let route = ctx.link().route::<Route>().unwrap_or(Route::Home);

        /* if route == Route::OnlineMultiplayer {
            let query_string = ctx.link().location().expect("no location").search();
            let lobby = query_string.split("=").collect::<Vec<&str>>()[1];
        } */

        html! {
            <>
                <div class={ "board-background" }>
                    {(0..BOARD_WIDTH).into_iter().map(|num| { // Create Columns for the Board
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
                <GameControlButtons board={ Rc::clone(&self.board) }
                    rerender_board_callback={ rerender_board_callback.clone() } />
            </>
        }
    }
}
