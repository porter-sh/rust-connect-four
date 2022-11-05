use crate::router;
use crate::util::board_state::BoardState;
use crate::util::util::DiskColor;
use crate::constants::*;
use gloo::console::{error, log};
use yew::{function_component, html, Callback, MouseEvent, Properties};
use yew_router::prelude::*;

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Properties, PartialEq)]
pub struct BackButtonProperties {
    pub disks: Rc<RefCell<BoardState>>,
    pub rerender_board_callback: Callback<MouseEvent>
}

#[function_component(UndoButton)]
pub fn undo_button(props: &BackButtonProperties) -> Html {

    return html! {

        <button class={ "control-btn" } onclick={
            let board = Rc::clone(&props.disks);
            let rerender_board_callback = props.rerender_board_callback.clone();
            Callback::from(move |_| {
                {

                    let mut disks = board.borrow_mut();
                    disks.game_won = false;

                    disks.current_player = if disks.current_player == DiskColor::P1 {DiskColor::P2} else {DiskColor::P1};

                    disks.num_moves -= 1;

                    let col = disks.game_history[disks.num_moves];
                    for row in (0..BOARD_HEIGHT).rev() {

                        if disks.board_state[row][col] != DiskColor::Empty {
                            disks.board_state[row][col] = DiskColor::Empty;
                            break;
                        }

                    }

                }

                rerender_board_callback.emit(MouseEvent::new("mousedown").unwrap());

            })
        }>
            { "Undo" }
        </button>

    };
        
}