//! board_state.position contains BoardState, which stores board representation and additional state
use crate::ai::ai::AI as AITrait;
use crate::util::util::{DiskColor, Disks, SecondPlayerExtension};
use constants::*;

use gloo::console::error;
use SecondPlayerExtension::{None, OnlinePlayer, SurvivalMode, AI};

/// BoardState stores the internal board representation, as well as other state data that other
/// board components use
///
/// Manually impls PartialEq since SplitSink does not impl PartialEq
pub struct BoardState {
    pub board_state: Disks,
    pub can_move: bool,
    pub current_player: DiskColor,
    pub game_history: [u8; (BOARD_WIDTH * BOARD_HEIGHT) as usize],
    pub num_moves: u8,
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
            game_history: [0u8; (BOARD_WIDTH * BOARD_HEIGHT) as usize],
            num_moves: 0u8,
            second_player_extension: None,
        }
    }
}

/// Implements functions to check if the game has been won
impl BoardState {
    /// Does everything required for the next player to make a move in the given column.
    pub fn make_move(&mut self, col: u8) {
        self.board_state.drop_disk(col).unwrap();
        self.update_can_move_if_won(); // must be called before dropping the disk
        self.update_player();
        self.update_game_history(col);
    }

    /// If playing online, send a message to the server containing the move, and whether
    /// the game was won.
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

    /// If playing singleplayer, give the AI a turn.
    pub fn run_ai_if_applicable(&mut self) {
        if self.second_player_extension.is_ai() || self.second_player_extension.is_survival_mode() {
            if let AI(ai) = &self.second_player_extension {
                if self.can_move {
                    let col = ai.get_move(&self.board_state, self.current_player);
                    if !self.board_state.is_col_full(col) {
                        self.make_move(col);
                    }
                }
            } else if let SurvivalMode(ai) = &mut self.second_player_extension {
                if self.can_move {
                    let col = ai.get_move(&self.board_state, self.current_player);
                    if !self.board_state.is_col_full(col) {
                        self.make_move(col);
                    }
                } else {
                    ai.increment_look_ahead();
                    self.board_state = Disks::default();
                    self.game_history = [0u8; (BOARD_WIDTH * BOARD_HEIGHT) as usize];
                    self.num_moves = 0;
                    self.can_move = true;
                }
            }
        }
    }

    /// If not online, set the current player to the next player.
    fn update_player(&mut self) {
        if !self.second_player_extension.is_online_player() {
            self.current_player = match self.current_player {
                DiskColor::P1 => DiskColor::P2,
                DiskColor::P2 => DiskColor::P1,
                _ => panic!("Invalid player color"),
            };
        }
    }

    /// Update the history of moves with the next move.
    fn update_game_history(&mut self, selected_col: u8) {
        self.game_history[self.num_moves as usize] = selected_col;
        self.num_moves += 1;
        if self.num_moves == BOARD_WIDTH * BOARD_HEIGHT {
            self.can_move = false;
        }
    }

    /// Check if the game has been won, and if so, set can_move to false.
    fn update_can_move_if_won(&mut self) {
        if self.board_state.check_last_drop_won() {
            self.can_move = false;
        }
    }
}
