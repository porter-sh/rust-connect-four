//! Contains the RandomAI implementation.
//! RandomAI randomly chooses any open column.
use super::{ai::AI, util};
use crate::util::util::{DiskColor, Disks};
use constants::BOARD_WIDTH;

pub struct RandomAI;

impl AI for RandomAI {
    /// Gets a random move from the available columns.
    fn get_move(&mut self, board: &Disks, _: DiskColor) -> u8 {
        // find which columns are open
        let mut available_cols = Vec::with_capacity(BOARD_WIDTH as usize);
        for col in 0..(BOARD_WIDTH as u8) {
            if !board.is_col_full(col) {
                available_cols.push(col);
            }
        }
        // chose one of the available columns at random
        match util::random_col_from_options(&available_cols) {
            Some(col) => *col,
            None => panic!("Fix the random AI, should have returned a value"),
        }
    }
}
