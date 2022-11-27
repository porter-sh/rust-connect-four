//! board contains the Board component
//! Board contains the internal BoardState, and renders that state through Column components
//! Board also accepts user input when in the middle of a game via Column components

use crate::{
    ai::{perfect::PerfectAI, random::RandomAI},
    components::{column::*, game_control_buttons::GameControlButtons},
    router::{AIRoute, Route},
    util::{board_state::BoardState, net::ServerMessage},
};
use constants::*;
use std::{cell::RefCell, rc::Rc};
use yew::{html, Component, Context, Html};
use yew_router::{prelude::*, scope_ext::HistoryHandle};

pub enum BoardMessages {
    Rerender,
    RerenderAndUpdateBoard(ServerMessage),
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
        let rerender_board_callback = ctx.link().callback(|_| BoardMessages::Rerender);
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
                <GameControlButtons board={ Rc::clone(&self.board) }
                    rerender_board_callback={ rerender_board_callback.clone() } />
            </>
        }
    }
}

impl Board {
    pub fn new(ctx: &Context<Board>) -> Self {
        let board_origin = Rc::new(RefCell::new(BoardState::new(
            ctx.link()
                .callback(|msg: ServerMessage| BoardMessages::RerenderAndUpdateBoard(msg)),
        )));
        Self {
            board: Rc::clone(&board_origin),
            _history_handle: Self::get_history_handle(ctx, board_origin),
        }
    }

    fn get_history_handle(ctx: &Context<Board>, board: Rc<RefCell<BoardState>>) -> HistoryHandle {
        /* let callback = ctx
        .link()
        .callback(|col_num: u8| BoardMessages::RerenderAndUpdateColumn(col_num));*/
        ctx.link()
            .add_history_listener(ctx.link().callback(move |history: AnyHistory| {
                let board_clone = Rc::clone(&board);
                // Will rerender the Board
                Self::on_reroute(board_clone, history.location());
                BoardMessages::Rerender
            }))
            .unwrap()
    }

    fn on_reroute(board: Rc<RefCell<BoardState>>, location: AnyLocation) {
        if let Some(route) = location.route::<Route>() {
            match route {
                Route::LocalMultiplayer => {
                    board.borrow_mut().reset(); // Reset the BoardState when starting a new game
                }
                Route::OnlineMultiplayer => {
                    let query_string = location.search();
                    let lobby = query_string.split("=").collect::<Vec<&str>>()[1];
                    board.borrow_mut().init_online(lobby.to_string());
                }
                Route::VersusBot => {
                    match location.route::<AIRoute>().unwrap_or(AIRoute::Random) {
                        AIRoute::Random => board.borrow_mut().init_ai(Box::new(RandomAI)),
                        AIRoute::Perfect => {
                            board.borrow_mut().init_ai(Box::new(PerfectAI::new(10)))
                        }
                        AIRoute::Survival => board
                            .borrow_mut()
                            .init_survival(Box::new(PerfectAI::new(1))),
                    };
                }
                _ => board
                    .borrow_mut()
                    .second_player_extension
                    .remove_extension(),
            }
        }
    }
}
