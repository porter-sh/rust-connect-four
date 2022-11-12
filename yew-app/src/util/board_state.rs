//! board_state contains BoardState, which stores board representation and additional state
use crate::util::util::{DiskColor, DiskData, Disks, SecondPlayerExtension};
use constants::*;
use std::cmp::min;

use gloo::console::error;

use SecondPlayerExtension::{None, OnlinePlayer, AI};

/// BoardState stores the internal board representation, as well as other state data that other
/// board components use
///
/// Manually impls PartialEq since SplitSink does not impl PartialEq
pub struct BoardState {
    pub board_state: Disks,
    pub current_player: DiskColor,
    pub can_move: bool,
    pub game_history: [usize; BOARD_WIDTH * BOARD_HEIGHT],
    pub num_moves: usize,
    pub second_player_extension: SecondPlayerExtension,
}

/// Manual PartialEq impl since SplitSink does not impl PartialEq
impl PartialEq for BoardState {
    fn eq(&self, other: &Self) -> bool {
        self.board_state == other.board_state
            && self.current_player == other.current_player
            && self.can_move == other.can_move
            && match (
                &self.second_player_extension,
                &other.second_player_extension,
            ) {
                (OnlinePlayer(_), OnlinePlayer(_)) | (AI(_), AI(_)) | (None, None) => true,
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
            can_move: true,
            game_history: [0usize; BOARD_WIDTH * BOARD_HEIGHT],
            num_moves: 0usize,
            second_player_extension: None,
        }
    }
}

/// Implements functions to check if the game has been won
impl BoardState {
    /// Returns if the game is won by dropping a disk in the location stored by DiskData
    pub fn check_winner(&mut self, new_disk: DiskData) {
        // check for a vertical win
        if new_disk.row < BOARD_HEIGHT - 3
            && self.board_state[new_disk.row + 1][new_disk.col] == new_disk.color
            && self.board_state[new_disk.row + 2][new_disk.col] == new_disk.color
            && self.board_state[new_disk.row + 3][new_disk.col] == new_disk.color
        {
            self.can_move = false;
        }

        // check for a win in other directions
        if Self::check_lateral(&self, &new_disk)
            || Self::check_right_diag(&self, &new_disk)
            || Self::check_left_diag(&self, &new_disk)
        {
            self.can_move = false;
        }
    }

    pub fn update_player(&mut self) {
        if !self.second_player_extension.is_online_player() {
            self.current_player = match self.current_player {
                DiskColor::P1 => DiskColor::P2,
                DiskColor::P2 => DiskColor::P1,
                _ => panic!("Invalid player color"),
            };
        }
    }

    pub fn update_game_history(&mut self, selected_col: usize) {
        self.game_history[self.num_moves] = selected_col;
        self.num_moves += 1;
        if self.num_moves == BOARD_WIDTH * BOARD_HEIGHT {
            self.can_move = false;
        }
    }

    pub fn update_server_if_online(&mut self, selected_col: usize) {
        if self.second_player_extension.is_online_player() {
            let mut col_num_addition = 0;
            if !self.can_move {
                col_num_addition = ConnectionProtocol::WINNING_MOVE_ADDITION;
            } else {
                self.can_move = false;
            }
            if let OnlinePlayer(sender) = &self.second_player_extension {
                if let Err(e) = sender.send(selected_col as u8 + col_num_addition) {
                    error!(format!("Failed to send message: {}", e));
                }
            }
        }
    }

    pub fn run_ai_if_applicable(&mut self) {
        if self.can_move && self.second_player_extension.is_ai() {
            if let AI(ai) = &self.second_player_extension {
                let col = ai.get_move(&self.board_state);
                for row in (0..BOARD_HEIGHT).rev() {
                    if self.board_state[row][col] == DiskColor::Empty {
                        self.board_state[row][col] = self.current_player;

                        let new_disk = DiskData::new(row, col, self.current_player);

                        self.check_winner(new_disk);
                        self.update_player();
                        self.update_game_history(col);

                        return;
                    }
                }
            }
        }
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
