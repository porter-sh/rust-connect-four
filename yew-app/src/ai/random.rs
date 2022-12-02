//! Contains the RandomAI implementation.
//! RandomAI randomly chooses any open column.
use super::{ai::AI, util};
use crate::util::disks::Disks;
use constants::BOARD_WIDTH;

pub struct RandomAI;

impl AI for RandomAI {
    /// Gets a random move from the available columns.
    fn request_move(&self, disks: &Disks) -> u8 {
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
                return *col;
            }
            _ => panic!("Fix the random AI, should have returned a value"),
        }
    }
}
