//! board_state.position contains BoardState, which stores board representation and additional state
use crate::{
    util::net::ServerMessage::{self, BoardState as BoardStateMessage, SimpleMessage},
    util::{
        second_player_extension::SecondPlayerExtension,
        util::{DiskColor, Disks, SecondPlayerAIMode, SecondPlayerSurvivalAIMode},
    },
};
use constants::*;
use yew::Callback;

use gloo::console::log;

/// BoardState stores the internal board representation, as well as other state data that other
/// board components use
///
/// Manually impls PartialEq since SplitSink does not impl PartialEq
#[derive(PartialEq)]
pub struct BoardState {
    pub disks: Disks,
    pub can_move: bool,
    pub current_player: DiskColor,
    pub game_history: [u8; (BOARD_WIDTH * BOARD_HEIGHT) as usize],
    pub num_moves: u8,
    pub second_player_extension: SecondPlayerExtension,
}

/// Implements functions to check if the game has been won
impl BoardState {
    /// Creates a new empty board, with a callback for rerendering the board.
    pub fn new(rerender_board_callback: Callback<ServerMessage>) -> Self {
        BoardState {
            disks: Disks::default(),
            can_move: true,
            current_player: DiskColor::P1,
            game_history: [0; (BOARD_WIDTH * BOARD_HEIGHT) as usize],
            num_moves: 0,
            second_player_extension: SecondPlayerExtension::new(rerender_board_callback),
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
    }

    /// Does everything required for the next player to make a move in the given column.
    pub fn make_move(&mut self, col: u8) -> Result<(), String> {
        self.disks.drop_disk(col)?;
        self.update_can_move_if_won(); // must be called before dropping the disk
        self.update_player_if_not_online();
        self.update_game_history(col);
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
    pub fn make_move_and_handoff_to_second_player(
        &mut self,
        selected_col: u8,
    ) -> Result<(), String> {
        self.make_move(selected_col)?;
        // Handoff to second player
        self.handoff_to_second_player(selected_col)
    }

    pub fn handoff_to_second_player(&mut self, selected_col: u8) -> Result<(), String> {
        if self
            .second_player_extension
            .request_move(selected_col, self)?
            && selected_col != ConnectionProtocol::UNDO
        {
            self.can_move = false;
        }
        Ok(())
    }

    /// Handles all the board changes based on a message from the second player.
    pub fn update_state_from_second_player_message(&mut self, msg: ServerMessage) {
        log!(format!("Received {:?}", msg));
        match msg {
            BoardStateMessage(update) => {
                // if the message is a non-winning move, it will be the client's turn next, so they can move
                if !update.game_won && self.current_player != DiskColor::Empty {
                    self.can_move = update.is_p1_turn == (self.current_player == DiskColor::P1);
                }
                // update the board
                self.disks = Disks::from(update);
            }

            SimpleMessage(msg) => {
                // initialization, telling the client which player they are
                if msg == ConnectionProtocol::IS_PLAYER_1 {
                    self.current_player = DiskColor::P1;
                } else if msg == ConnectionProtocol::IS_PLAYER_2 {
                    self.current_player = DiskColor::P2;
                    self.can_move = false;
                } else if msg == ConnectionProtocol::IS_SPECTATOR {
                    self.current_player = DiskColor::Empty;
                    self.can_move = false;
                }

                if msg < 7 {
                    self.make_move(msg).unwrap();
                    self.can_move = true;
                }
            }

            _ => panic!("Received invalid update message from the task reading from the server."),
        }
    }

    // Resets the board, and requests a server connection.
    pub fn init_online(&mut self, lobby: String) {
        self.reset(); // reset board data
        self.second_player_extension.init_online(lobby); // set the second player to be online
    }

    // Resets the board, and extends with an AI as the second player.
    pub fn init_ai(&mut self, ai_type: SecondPlayerAIMode) {
        self.reset(); // reset board data
        self.second_player_extension.init_ai(ai_type); // set the second player to be an AI
    }

    // Resets the board, and extends to survival mode (second player is AI that improves each round).
    pub fn init_survival(&mut self, ai_type: SecondPlayerSurvivalAIMode) {
        self.reset(); // reset board data
        self.second_player_extension.init_survival(ai_type); // set the second player to be an AI
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

    /// Check if the game has been won, and if so, set can_move to false.
    fn update_can_move_if_won(&mut self) {
        if self.disks.check_last_drop_won() {
            self.can_move = false;
        }
    }
}
