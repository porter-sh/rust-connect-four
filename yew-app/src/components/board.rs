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

pub struct Board {
    board: Rc<RefCell<BoardState>>,
    #[allow(unused)]
    history_handle: HistoryHandle,
}

pub enum BoardMessages {
    Rerender,
    NoChange,
}

impl Component for Board {
    type Message = BoardMessages;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let cb: yew::Callback<MouseEvent> = ctx.link().callback(|_| {
            log!("In callback");
            BoardMessages::Rerender
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
                .add_history_listener(ctx.link().callback(move |history: AnyHistory| {
                    if let Some(route) = history.location().route::<router::Route>() {
                        match route {
                            router::Route::LocalMultiplayer
                            | router::Route::VersusBot
                            | router::Route::OnlineMultiplayer => {
                                *board.borrow_mut() = Default::default();

                                let mut ws = WebSocket::open("ws://127.0.0.1:8081").unwrap();
                                let (mut write, mut read) = ws.split();
                            }
                            _ => {}
                        }
                    }
                    BoardMessages::Rerender
                }))
                .unwrap()
        };
        Self {
            board,
            history_handle,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            BoardMessages::Rerender => true,
            _ => false,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let rerender_board_callback = ctx.link().callback(|_| BoardMessages::Rerender);
        let route = ctx.link().route::<router::Route>().unwrap_or(router::Route::Home);
        html! {
            <>
                <div class={ "board-background" }>
                    {(0..BOARD_WIDTH).into_iter().map(|num| {
                        html! {
                            <Column col_num={ num } disks={ Rc::clone(&self.board) } in_game={
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
                            | router::Route::VersusBot => {
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
