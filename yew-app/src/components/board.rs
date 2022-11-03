use crate::components::column::*;
use crate::constants::*;
use crate::router;
use crate::util::board_state::BoardState;
use yew::{html, Component, Context, Html};
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
        let rerender_board_callback = ctx.link().callback(|_| BoardMessages::Rerender);
        html! {
            <div class={"board-background"}>
                {(0..BOARD_WIDTH).into_iter().map(|num| {
                    html! {
                        <Column col_num={ num } disks={ Rc::clone(&self.board) } in_game={
                            if let Some(route) = ctx.link().route::<router::Route>() {
                                match route {
                                    router::Route::LocalMultiplayer
                                        | router::Route::VersusBot
                                        | router::Route::OnlineMultiplayer => {
                                            true
                                        },
                                    _ => false
                                }
                            } else{
                                false
                            }
                        } rerender_board_callback={rerender_board_callback.clone()} />
                    }
                }).collect::<Html>()}
            </div>
        }
    }
}
