//! board_state.position contains BoardState, which stores board representation and additional state
use core::num;

use crate::{
    components::utility_bar::InfoMessage,
    util::{
        disks::Disks,
        second_player_extension::SecondPlayerExtension,
        util::GameUpdateMessage::{self, BoardState as BoardStateMessage, SimpleMessage},
        util::{DiskColor, SecondPlayerAIMode, SecondPlayerSurvivalAIMode},
    },
};
use constants::*;
use yew::Callback;

use gloo::console::log;

use super::util::SecondPlayerExtensionMode;

#[derive(PartialEq)]
pub enum RequestMoveResult {
    WillRerenderLater,
    RerenderNow(u8),
    NoRequestMade,
}

impl Default for RequestMoveResult {
    fn default() -> Self {
        Self::NoRequestMade
    }
}

/// BoardState stores the internal board representation, as well as other state data that other
/// board components use
///
/// Manually impls PartialEq since SplitSink does not impl PartialEq
#[derive(PartialEq)]
pub struct BoardState {
    pub disks: Disks,
    pub can_move: bool,
    // NOTE: although disks maintains an "is_p1_turn" field, that is used
    // for rendering the board, and "current_player" is used for game logic. The
    // two are not always the same.
    pub current_player: DiskColor,
    pub game_history: [u8; (BOARD_WIDTH * BOARD_HEIGHT) as usize],
    pub num_moves: u8,
    pub second_player_extension: SecondPlayerExtension,
    pub info_message: InfoMessage,
}

/// Implements functions to check if the game has been won
impl BoardState {
    /// Creates a new empty board, with a callback for rerendering the board.
    pub fn new(rerender_board_callback: Callback<GameUpdateMessage>) -> Self {
        BoardState {
            disks: Disks::default(),
            can_move: true,
            current_player: DiskColor::P1,
            game_history: [0; (BOARD_WIDTH * BOARD_HEIGHT) as usize],
            num_moves: 0,
            second_player_extension: SecondPlayerExtension::new(rerender_board_callback),
            info_message: InfoMessage::NoMessage,
        }
    }

    /// Resets the board to how it should be at the start of a new game.
    pub fn reset(&mut self) {
        self.disks = Disks::default();
        self.can_move = true;
        self.current_player = DiskColor::P1;
        self.game_history = [0; (BOARD_WIDTH * BOARD_HEIGHT) as usize];
        self.num_moves = 0;
        self.second_player_extension.remove_extension();
        self.info_message = InfoMessage::P1Turn;
    }

    /// Does everything required for the next player to make a move in the given column.
    pub fn make_move(&mut self, col: u8) -> Result<(), String> {
        self.disks.drop_disk(col)?;
        let game_won = self.update_can_move_if_won();
        self.update_player_if_not_online();
        self.update_game_history(col);
        // TODO FIX THIS LOGIC
        log!(format!(
            "game_won: {}, num_moves {}",
            game_won, self.num_moves
        ));
        self.update_info_message(game_won);
        log!(format!("Info message: {:?}", self.info_message));
        Ok(())
    }

    /// Given the desired move of the current player, update the board state. If
    /// there is a second player extension, send the move to the second player
    /// however the second player needs to receive the move. If there is not a
    /// second player, set up the game state to be ready for the second local
    /// player to take a turn.
    /// Additionally, if if there is a second player extension, request their
    /// move (NON-BLOCKING), then let the board wait for a response in the form
    /// of a callback.
    /// Returns Result of whether the second player extension will eventually
    /// call back with a move.
    pub fn make_move_and_handoff_to_second_player(
        &mut self,
        selected_col: u8,
    ) -> Result<RequestMoveResult, String> {
        self.make_move(selected_col)?;
        if !self.can_move && self.second_player_extension.is_survival_mode() {
            self.second_player_extension
                .increment_survival_mode_difficulty();
            self.disks = Disks::default();
            self.game_history = [0u8; (BOARD_WIDTH * BOARD_HEIGHT) as usize];
            self.num_moves = 0;
            self.can_move = true;
            Ok(RequestMoveResult::NoRequestMade)
        } else {
            // Handoff to second player
            self.handoff_to_second_player(selected_col)
        }
    }

    /// Undo the last move and sends a message to the second player.
    pub fn undo_move_and_handoff_to_second_player(&mut self) {
        // At the start of the game
        if self.num_moves == 0 {
            return;
        }

        if !self.second_player_extension.is_online_player() {
            // Revert to previous player
            self.current_player = if self.current_player == DiskColor::P1 {
                DiskColor::P2
            } else {
                DiskColor::P1
            };
        }

        self.can_move = true; // Undoes win, allowing board interaction
        self.num_moves -= 1;

        let num_moves = self.num_moves;

        let col = self.game_history[num_moves as usize]; // Get the column the last move was made in
        self.disks.rm_disk_from_col(col); // Remove the disk from the columns
        self.handoff_to_second_player(ConnectionProtocol::UNDO)
            .unwrap_or_default();
    }

    /// Returns a result of whether the second player extension will eventually
    /// call back with a move.
    pub fn handoff_to_second_player(
        &mut self,
        selected_col: u8,
    ) -> Result<RequestMoveResult, String> {
        let res = self
            .second_player_extension
            .request_move(selected_col, self)?;
        if selected_col != ConnectionProtocol::UNDO && res == RequestMoveResult::WillRerenderLater {
            self.can_move = false;
            Ok(res)
        } else {
            //////xxx
            Ok(res)
        }
    }

    /// Handles all the board changes based on a message from the second player.
    pub fn update_state_from_second_player_message(&mut self, msg: GameUpdateMessage) {
        log!(format!("Received {:?}", msg));
        match msg {
            BoardStateMessage(update) => {
                // if the message is a non-winning move, it will be the client's turn next, so they can move
                if !update.game_won {
                    if self.current_player != DiskColor::Empty {
                        self.can_move = update.is_p1_turn == (self.current_player == DiskColor::P1);
                    }
                    self.info_message = if update.is_p1_turn {
                        InfoMessage::P1Turn
                    } else {
                        InfoMessage::P2Turn
                    };
                } else {
                    self.info_message = if update.is_p1_turn {
                        InfoMessage::P2Win
                    } else {
                        InfoMessage::P1Win
                    };
                }
                // update the board
                self.disks = Disks::from(update);
            }

            SimpleMessage(msg) => {
                // initialization, telling the client which player they are
                match msg {
                    ConnectionProtocol::IS_PLAYER_1 => {
                        self.current_player = DiskColor::P1;
                        self.can_move = false;
                        self.info_message = InfoMessage::WaitingForOpponent;
                    }
                    ConnectionProtocol::IS_PLAYER_2 => {
                        self.current_player = DiskColor::P2;
                        self.can_move = false;
                        self.info_message = InfoMessage::P1Turn;
                    }
                    ConnectionProtocol::IS_SPECTATOR => {
                        self.current_player = DiskColor::Empty;
                        self.can_move = false;
                        // accounts for the fact that the spectator does not join at the beginning of the game.
                        if self.num_moves % 2 == 0 {
                            self.info_message = InfoMessage::P1Turn;
                        } else {
                            self.info_message = InfoMessage::P2Turn;
                        }
                    }
                    ConnectionProtocol::SECOND_PLAYER_CONNECTED => {
                        if self.current_player == DiskColor::P1 {
                            self.can_move = true;
                            self.info_message = InfoMessage::P1Turn;
                        }
                    }
                    ConnectionProtocol::CONNECTION_FAILURE => {
                        self.info_message = InfoMessage::ConnectionFailed;
                    }
                    ConnectionProtocol::COL_0..=ConnectionProtocol::COL_6 => {
                        self.make_move(msg).unwrap();
                        if !self.disks.check_last_drop_won() {
                            self.can_move = true;
                        }
                    }
                    _ => {}
                }
            }

            _ => panic!("Received invalid update message from the task reading from the server."),
        }
    }

    /// Resets the board, and requests a server connection.
    pub fn init_online(&mut self, lobby: String) {
        self.reset(); // reset board data
        self.info_message = InfoMessage::Connecting;
        self.second_player_extension.init_online(lobby); // set the second player to be online
        log!(format!(
            "Extension Mode: {}",
            match &self.second_player_extension.mode {
                SecondPlayerExtensionMode::OnlinePlayer { .. } => "online",
                SecondPlayerExtensionMode::AI(_) => "AI",
                SecondPlayerExtensionMode::SurvivalMode(_) => "survival",
                SecondPlayerExtensionMode::None => "none",
            }
        ));
        if self.second_player_extension.mode == SecondPlayerExtensionMode::None {
            self.info_message = InfoMessage::ConnectionFailed;
        }
    }

    /// Resets the board, and extends with an AI as the second player.
    pub fn init_ai(&mut self, ai_type: SecondPlayerAIMode) {
        self.reset(); // reset board data
        self.second_player_extension.init_ai(ai_type); // set the second player to be an AI
    }

    /// Resets the board, and extends to survival mode (second player is AI that improves each round).
    pub fn init_survival(&mut self, ai_type: SecondPlayerSurvivalAIMode) {
        self.reset(); // reset board data
        self.second_player_extension.init_survival(ai_type); // set the second player to be an AI
    }

    pub fn get_second_player_mode(&self) -> &SecondPlayerExtensionMode {
        &self.second_player_extension.mode
    }

    /// If not online, set the current player to the next player.
    fn update_player_if_not_online(&mut self) {
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

    fn update_info_message(&mut self, game_won: bool) {
        self.info_message = if if self.second_player_extension.is_online_player() {
            self.current_player == DiskColor::P2
        } else {
            self.num_moves % 2 == 0
        } {
            // P1 next
            if game_won {
                InfoMessage::P2Win
            } else {
                InfoMessage::P1Turn
            }
        } else {
            // P2 next
            if game_won {
                InfoMessage::P1Win
            } else {
                InfoMessage::P2Turn
            }
        };
    }

    /// Check if the game has been won, and if so, set can_move to false.
    /// Returns true if the game has been won.
    fn update_can_move_if_won(&mut self) -> bool {
        if self.disks.check_last_drop_won() {
            self.can_move = false;
            return true;
        }
        false
    }
}
