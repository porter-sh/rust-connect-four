//! undo_button contains the UndoButton component
//! The UndoButton reverts previous moves up until the start of the game

use crate::util::board_state::BoardState;
use crate::util::util::DiskColor;
use crate::constants::*;
use yew::{function_component, html, Callback, MouseEvent, Properties};

use std::cell::RefCell;
use std::rc::Rc;

/// Properties to allow the UndoButton to interact with other components
#[derive(Properties, PartialEq)]
pub struct UndoButtonProperties {
    pub disks: Rc<RefCell<BoardState>>, // Mutably share BoardState across components
    pub rerender_board_callback: Callback<MouseEvent> // Tells the Board component to rerender
}

/// UndoButton component
/// When clicked, reverts previous moves up until the start of the game
#[function_component(UndoButton)]
pub fn undo_button(props: &UndoButtonProperties) -> Html {

    return html! {

        <button class={ "control-btn" } onclick={
            let board = Rc::clone(&props.disks);
            let rerender_board_callback = props.rerender_board_callback.clone();
            Callback::from(move |_| {
                // Limit the scope of BoardState mutable borrow so other components can check the BoardState when they rerender
                {

                    let mut disks = board.borrow_mut();
                    if disks.num_moves == 0 { return; } // At the start of the game

                    disks.game_won = false; // Undoes win, allowing board interaction

                    // Revert to previous player
                    disks.current_player = if disks.current_player == DiskColor::P1 {DiskColor::P2} else {DiskColor::P1};

                    disks.num_moves -= 1;

                    let num_moves = disks.num_moves;

                    let col = disks.game_history[num_moves]; // Get the column the last move was made in
                    for row in 0..BOARD_HEIGHT {

                        if disks.board_state[row][col] != DiskColor::Empty { // First nonempty space is the last move in this column
                            disks.board_state[row][col] = DiskColor::Empty;
                            break;
                        }

                    }

                } // Mutable borrow of the BoardState dropped, so other components can check the BoardState when they rerender

                // Tell the Board to rerender
                rerender_board_callback.emit(MouseEvent::new("mousedown").unwrap()); // MouseEvent type irrelevant
            })
        }>
            { "Undo" }
        </button>

    };
        
}