//! board_state.position contains BoardState, which stores board representation and additional state
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
    pub game_history: [usize; BOARD_WIDTH * BOARD_HEIGHT],
    pub num_moves: usize,
    pub second_player_extension: SecondPlayerExtension,
}

/// Manual PartialEq impl since SplitSink does not impl PartialEq
impl PartialEq for BoardState {
    fn eq(&self, other: &Self) -> bool {
        self.board_state.position == other.board_state.position
            && self.current_player == other.current_player
            && self.board_state.can_move == other.board_state.can_move
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
            current_player: DiskColor::P1,
            game_history: [0usize; BOARD_WIDTH * BOARD_HEIGHT],
            num_moves: 0usize,
            second_player_extension: None,
            ..Default::default()
        }
    }
}

/// Implements functions to check if the game has been won
impl BoardState {
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
            self.board_state.can_move = false;
        }
    }

    pub fn update_server_if_online(&mut self, selected_col: usize) {
        if self.second_player_extension.is_online_player() {
            let mut col_num_addition = 0;
            if !self.board_state.can_move {
                col_num_addition = ConnectionProtocol::WINNING_MOVE_ADDITION;
            } else {
                self.board_state.can_move = false;
            }
            if let OnlinePlayer(sender) = &self.second_player_extension {
                if let Err(e) = sender.send(selected_col as u8 + col_num_addition) {
                    error!(format!("Failed to send message: {}", e));
                }
            }
        }
    }

    pub fn run_ai_if_applicable(&mut self) {
        if self.board_state.can_move && self.second_player_extension.is_ai() {
            if let AI(ai) = &self.second_player_extension {
                let col = ai.get_move(&self.board_state, self.current_player);
                for row in (0..BOARD_HEIGHT).rev() {
                    if self.board_state.position[row][col] == DiskColor::Empty {
                        self.board_state.position[row][col] = self.current_player;

                        let new_disk = DiskData::new(row, col, self.current_player);

                        self.board_state.check_winner(new_disk);
                        self.update_player();
                        self.update_game_history(col);

                        return;
                    }
                }
            }
        }
    }
}
