use crate::components::board_state::BoardState;
use crate::components::column::*;
use crate::constants::*;
use crate::router;
use gloo::console::log;
use yew::{html, /*Callback, Children,*/ Component, Context, Html, Properties};
use yew_router::prelude::*;
use yew_router::scope_ext::HistoryHandle;

use std::cell::RefCell;
use std::rc::Rc;

pub struct Board {
    board: Rc<RefCell<BoardState>>,
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
        html! {
            <div class={"board-background"}>
                {(0..BOARD_WIDTH).into_iter().map(|num| {
                    html! {
                        <Column col_num={ num } disks={ Rc::clone(&self.board) } in_game={
                            if let Some(location) = ctx.link().location() {
                                if let Some(route) = location.route::<router::Route>() {
                                    match route {
                                        router::Route::LocalMultiplayer
                                            | router::Route::VersusBot
                                            | router::Route::OnlineMultiplayer => {
                                                true
                                            },
                                        _ => false
                                    }
                                } else {
                                    false
                                }
                            } else{
                                false
                            }
                        } />
                    }
                }).collect::<Html>()}
            </div>
        }
    }
}
