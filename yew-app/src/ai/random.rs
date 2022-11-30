//! Contains the RandomAI implementation.
//! RandomAI randomly chooses any open column.
use super::{ai::AI, util};
use crate::util::{net::ServerMessage, util::Disks};
use constants::BOARD_WIDTH;
use yew::Callback;

pub struct RandomAI {
    rerender_board_callback: Callback<ServerMessage>,
}

impl RandomAI {
    pub fn new(rerender_board_callback: Callback<ServerMessage>) -> Self {
        Self {
            rerender_board_callback,
        }
    }
}

impl AI for RandomAI {
    /// Gets a random move from the available columns.
    fn request_move(&self, disks: &Disks) {
        // find which columns are open
        let mut available_cols = Vec::with_capacity(BOARD_WIDTH as usize);
        for col in 0..(BOARD_WIDTH as u8) {
            if !disks.is_col_full(col) {
                available_cols.push(col);
            }
        }
        // chose one of the available columns at random
        match util::random_col_from_options(&available_cols) {
            Some(col) => {
                self.rerender_board_callback
                    .emit(ServerMessage::SimpleMessage(*col));
            }
            _ => panic!("Fix the random AI, should have returned a value"),
        }
    }
}
