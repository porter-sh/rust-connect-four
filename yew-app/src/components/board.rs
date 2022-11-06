//! board contains the Board component
//! Board contains the internal BoardState, and renders that state through Column components
//! Board also accepts user input when in the middle of a game via Column components

use crate::components::{column::*, undo_button::*};
use crate::constants::*;
use crate::router;
use crate::util::board_state::BoardState;
use yew::{html, Component, Context, Html};
use yew_router::prelude::*;
use yew_router::scope_ext::HistoryHandle;

use yew::MouseEvent;

use futures::{SinkExt, StreamExt};
use gloo::console::log;
use gloo_net::websocket::{futures::WebSocket, Message};
use wasm_bindgen_futures::spawn_local;

use std::cell::RefCell;
use std::rc::Rc;

/// Board component to store state of the board, to render the board, and to accept user input
pub struct Board {
    board: Rc<RefCell<BoardState>>, // Mutably share BoardState across components
    #[allow(unused)]
    history_handle: HistoryHandle // when not dropped allows the Board to respond to route changes
}

impl Component for Board {
    type Message = ();
    type Properties = ();

    /// Creates the Board component and adds a history listener to selectively react to and rerender on route changes
    fn create(ctx: &Context<Self>) -> Self {
        let cb: yew::Callback<MouseEvent> = ctx.link().callback(|_| {
            log!("In callback");
        });

        let mut ws = WebSocket::open("ws://127.0.0.1:8081").unwrap();
        let (mut write, mut read) = ws.split();

        spawn_local(async move {
            while let Some(msg) = read.next().await {
                cb.emit(MouseEvent::new("mousedown").unwrap());
                log!(format!("1. {:?}", msg))
            }
            log!("WebSocket Closed")
        });

        let board = Rc::new(RefCell::new(Default::default()));
        let history_handle = {
            let board = Rc::clone(&board);
            ctx.link()
                .add_history_listener(ctx.link().callback(move |history: AnyHistory| { // Will rerender the Board
                    if let Some(route) = history.location().route::<router::Route>() {
                        match route {
                            router::Route::LocalMultiplayer
                            | router::Route::VersusBot
                            | router::Route::OnlineMultiplayer => {
                                *board.borrow_mut() = Default::default(); // Reset the BoardState when starting a new game

                                let mut ws = WebSocket::open("ws://127.0.0.1:8081").unwrap();
                                let (mut write, mut read) = ws.split();
                            }
                            _ => {}
                        }
                    }
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
    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    /// Renders the Board
    /// If in the middle of a game, allows for user input
    /// Renders an UndoButton if playing a supported gamemode
    fn view(&self, ctx: &Context<Self>) -> Html {
        let rerender_board_callback = ctx.link().callback(|_| ());
        let route = ctx.link().route::<router::Route>().unwrap_or(router::Route::Home);
        html! {
            <>
                <div class={ "board-background" }>
                    {(0..BOARD_WIDTH).into_iter().map(|num| { // Create Columns for the Board
                        html! {
                            <Column col_num={ num } disks={ Rc::clone(&self.board) } in_game={ // Accept input if in game
                                match route {
                                    router::Route::LocalMultiplayer
                                        | router::Route::VersusBot
                                        | router::Route::OnlineMultiplayer => {
                                            true
                                        },
                                    _ => false
                                }
                            } rerender_board_callback={ rerender_board_callback.clone() } />
                        }
                    }).collect::<Html>()}
                </div>
                <div class={ "control-container" }>
                    {match route {
                        router::Route::LocalMultiplayer
                            | router::Route::VersusBot => { // Render an UndoButton if playing a supported gamemode
                                html! {
                                    <UndoButton
                                        disks={ Rc::clone(&self.board) }
                                        rerender_board_callback={ rerender_board_callback.clone() }
                                    />
                                }
                            }
                        _ => html! {}
                    }}
                </div>
            </>
        }
    }
}
