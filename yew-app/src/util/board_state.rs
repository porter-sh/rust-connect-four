//! board_state contains BoardState, which stores board representation and additional state

use crate::constants::*;
use crate::util::util::{DiskColor, DiskData, Disks};
use tokio::sync::mpsc::UnboundedSender;

use std::cmp::min;

/// BoardState stores the internal board representation, as well as other state data that other
/// board components use
///
/// Manually impls PartialEq since SplitSink does not impl PartialEq
pub struct BoardState {
    pub board_state: Disks,
    pub current_player: DiskColor,
    pub game_won: bool,
    pub game_history: [usize; BOARD_WIDTH * BOARD_HEIGHT],
    pub num_moves: usize,
    pub socket_writer: Option<UnboundedSender<u8>>,
}

/// Manual PartialEq impl since SplitSink does not impl PartialEq
impl PartialEq for BoardState {
    fn eq(&self, other: &Self) -> bool {
        self.board_state == other.board_state
            && self.current_player == other.current_player
            && self.game_won == other.game_won
            && match (&self.socket_writer, &other.socket_writer) {
                (Some(_), Some(_)) | (None, None) => true,
                _ => false,
            }
    }
}

/// Construct a default BoardState, useful when starting a new game
impl Default for BoardState {
    fn default() -> Self {
        Self {
            board_state: [[DiskColor::Empty; BOARD_WIDTH]; BOARD_HEIGHT],
            current_player: DiskColor::P1,
            game_won: false,
            game_history: [0usize; BOARD_WIDTH * BOARD_HEIGHT],
            num_moves: 0usize,
            socket_writer: None,
        }
    }
}

/// Implements functions to check if the game has been won
impl BoardState {
    /// Returns if the game is won by dropping a disk in the location stored by DiskData
    pub fn check_winner(&self, new_disk: DiskData) -> bool {
        // check for a vertical win
        if new_disk.row < BOARD_HEIGHT - 3
            && self.board_state[new_disk.row + 1][new_disk.col] == new_disk.color
            && self.board_state[new_disk.row + 2][new_disk.col] == new_disk.color
            && self.board_state[new_disk.row + 3][new_disk.col] == new_disk.color
        {
            return true;
        }

        // check for a win in other directions
        if Self::check_lateral(&self, &new_disk)
            || Self::check_right_diag(&self, &new_disk)
            || Self::check_left_diag(&self, &new_disk)
        {
            return true;
        }

        false
    }

    /// Helper function to check_winner
    /// Returns whether a horizontal win occured
    fn check_lateral(&self, new_disk: &DiskData) -> bool {
        let mut left_count = 0;
        while left_count < new_disk.left_range {
            if self.board_state[new_disk.row][new_disk.col - 1 - left_count] != new_disk.color {
                break;
            }
            left_count += 1;
        }
        if left_count == 3 {
            return true;
        }

        let mut right_count = 0;
        while right_count < new_disk.right_range {
            if self.board_state[new_disk.row][new_disk.col + 1 + right_count] != new_disk.color {
                break;
            }
            right_count += 1;
            if left_count + right_count == 3 {
                return true;
            }
        }
        false
    }

    /// Helper function to check_winner
    /// Returns whether a right diagonal win occured
    fn check_right_diag(&self, new_disk: &DiskData) -> bool {
        let mut top_left_count = 0;
        while top_left_count < min(new_disk.up_range, new_disk.left_range) {
            if self.board_state[new_disk.row - 1 - top_left_count]
                [new_disk.col - 1 - top_left_count]
                != new_disk.color
            {
                break;
            }
            top_left_count += 1;
        }
        if top_left_count == 3 {
            return true;
        }

        let mut bottom_right_count = 0;
        while bottom_right_count < min(new_disk.down_range, new_disk.right_range) {
            if self.board_state[new_disk.row + 1 + bottom_right_count]
                [new_disk.col + 1 + bottom_right_count]
                != new_disk.color
            {
                break;
            }
            bottom_right_count += 1;
            if top_left_count + bottom_right_count == 3 {
                return true;
            }
        }
        false
    }

    /// Helper function to check_winner
    /// Returns whether a left diagonal win occured
    fn check_left_diag(&self, new_disk: &DiskData) -> bool {
        let mut top_right_count = 0;
        while top_right_count < min(new_disk.up_range, new_disk.right_range) {
            if self.board_state[new_disk.row - 1 - top_right_count]
                [new_disk.col + 1 + top_right_count]
                != new_disk.color
            {
                break;
            }
            top_right_count += 1;
        }
        if top_right_count == 3 {
            return true;
        }

        let mut bottom_left_count = 0;
        while bottom_left_count < min(new_disk.down_range, new_disk.left_range) {
            if self.board_state[new_disk.row + 1 + bottom_left_count]
                [new_disk.col - 1 - bottom_left_count]
                != new_disk.color
            {
                break;
            }
            bottom_left_count += 1;
            if top_right_count + bottom_left_count == 3 {
                return true;
            }
        }
        false
    }
}
