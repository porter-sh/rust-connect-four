//! board_state.position contains BoardState, which stores board representation and additional state
use crate::util::util::{DiskColor, Disks, SecondPlayerExtension};
use constants::*;

use gloo::console::error;
use SecondPlayerExtension::{None, OnlinePlayer, AI};

/// BoardState stores the internal board representation, as well as other state data that other
/// board components use
///
/// Manually impls PartialEq since SplitSink does not impl PartialEq
pub struct BoardState {
    pub board_state: Disks,
    pub can_move: bool,
    pub current_player: DiskColor,
    pub game_history: [u8; BOARD_WIDTH * BOARD_HEIGHT],
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
            board_state: Disks::default(),
            can_move: true,
            current_player: DiskColor::P1,
            game_history: [0u8; BOARD_WIDTH * BOARD_HEIGHT],
            num_moves: 0usize,
            second_player_extension: None,
        }
    }
}

/// Implements functions to check if the game has been won
impl BoardState {
    pub fn make_move(&mut self, col: u8) {
        self.update_can_move_if_will_win(col);
        self.board_state
            .drop_disk(col, &self.current_player)
            .unwrap();
        self.update_player();
        self.update_game_history(col);
    }

    pub fn update_server_if_online(&mut self, selected_col: u8) {
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
                let col = ai.get_move(&self.board_state, self.current_player);
                if !self.board_state.is_col_full(col) {
                    self.make_move(col);
                }
            }
        }
    }

    fn update_player(&mut self) {
        if !self.second_player_extension.is_online_player() {
            self.current_player = match self.current_player {
                DiskColor::P1 => DiskColor::P2,
                DiskColor::P2 => DiskColor::P1,
                _ => panic!("Invalid player color"),
            };
        }
    }

    fn update_game_history(&mut self, selected_col: u8) {
        self.game_history[self.num_moves] = selected_col;
        self.num_moves += 1;
        if self.num_moves == BOARD_WIDTH * BOARD_HEIGHT {
            self.can_move = false;
        }
    }

    fn update_can_move_if_will_win(&mut self, col: u8) {
        if self.board_state.check_winner(col, &self.current_player) {
            self.can_move = false;
        }
    }
}
