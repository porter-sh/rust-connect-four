//! board contains the Board component
//! Board contains the internal BoardState, and renders that state through Column components
//! Board also accepts user input when in the middle of a game via Column components

use crate::{
    ai::{perfect::PerfectAI, random::RandomAI},
    components::{column::*, game_control_buttons::GameControlButtons},
    router::{AIRoute, Route},
    util::{board_state::BoardState, net, util::DiskColor},
};
use constants::*;
use std::{cell::RefCell, rc::Rc};
use yew::{html, Component, Context, Html};
use yew_router::{prelude::*, scope_ext::HistoryHandle};

use gloo::console::log;

use crate::util::util::SecondPlayerExtension::{None, OnlinePlayer, AI};

pub enum BoardMessages {
    Rerender,
    RerenderAndUpdateColumn(u8),
}

/// Board component to store state of the board, to render the board, and to accept user input
pub struct Board {
    board: Rc<RefCell<BoardState>>, // Mutably share BoardState across components
    _history_handle: HistoryHandle, // when not dropped allows the Board to respond to route changes
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
        if let BoardMessages::RerenderAndUpdateColumn(mut msg_val) = msg {
            let mut board = self.board.borrow_mut();
            if msg_val == ConnectionProtocol::IS_PLAYER_1 {
                board.current_player = DiskColor::P1;
            } else if msg_val == ConnectionProtocol::IS_PLAYER_2 {
                board.current_player = DiskColor::P2;
                board.can_move = false;
            } else if ConnectionProtocol::COL_0 + ConnectionProtocol::WINNING_MOVE_ADDITION
                <= msg_val
                && msg_val <= ConnectionProtocol::COL_6 + ConnectionProtocol::WINNING_MOVE_ADDITION
            {
                msg_val -= ConnectionProtocol::WINNING_MOVE_ADDITION;
            } else {
                board.can_move = true;
            }
            if ConnectionProtocol::COL_0 <= msg_val && msg_val <= ConnectionProtocol::COL_6 {
                board.board_state.drop_disk(msg_val).unwrap(); // TODO: Handle error
            }
            log!(format!("Received {}", msg_val));
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
                <GameControlButtons board={ Rc::clone(&self.board) }
                    rerender_board_callback={ rerender_board_callback.clone() } />
            </>
        }
    }
}

impl Board {
    pub fn new(ctx: &Context<Board>) -> Self {
        let board_origin = Rc::new(RefCell::new(Default::default()));
        Self {
            board: Rc::clone(&board_origin),
            _history_handle: Self::get_history_handle(ctx, board_origin),
        }
    }

    fn get_history_handle(ctx: &Context<Board>, board: Rc<RefCell<BoardState>>) -> HistoryHandle {
        let callback = ctx
            .link()
            .callback(|col_num: u8| BoardMessages::RerenderAndUpdateColumn(col_num));
        ctx.link()
            .add_history_listener(ctx.link().callback(move |history: AnyHistory| {
                let board_clone = Rc::clone(&board);
                // Will rerender the Board
                Self::on_reroute(board_clone, callback.clone(), history);
                BoardMessages::Rerender
            }))
            .unwrap()
    }

    fn on_reroute(
        board: Rc<RefCell<BoardState>>,
        callback: yew::Callback<u8>,
        history: AnyHistory,
    ) {
        let location = history.location();
        if let Some(route) = location.route::<Route>() {
            match route {
                Route::LocalMultiplayer => {
                    *board.borrow_mut() = Default::default(); // Reset the BoardState when starting a new game
                }
                Route::OnlineMultiplayer => {
                    *board.borrow_mut() = BoardState {
                        second_player_extension: match net::spawn_connection_threads(
                            callback.clone(),
                        ) {
                            Ok(writer) => OnlinePlayer(writer),
                            _ => None,
                        },
                        ..Default::default()
                    };
                }
                Route::VersusBot => {
                    *board.borrow_mut() = BoardState {
                        second_player_extension: match location
                            .route::<AIRoute>()
                            .unwrap_or(AIRoute::Random)
                        {
                            AIRoute::Random => AI(Box::new(RandomAI)),
                            AIRoute::Perfect => AI(Box::new(PerfectAI::new(10))),
                        },
                        ..Default::default()
                    }
                }
                _ => board.borrow_mut().second_player_extension = None,
            }
        }
    }
}
