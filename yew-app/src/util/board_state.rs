//! board_state.position contains BoardState, which stores board representation and additional state
use crate::{
    ai::ai::{AI as AITrait, SurvivalAI},
    util::{
        second_player_extension::{SecondPlayerExtension, SecondPlayerExtensionMode},
        util::{DiskColor, Disks},
    },
};
use constants::*;

use gloo::console::{error, log};
use yew::Callback;

use SecondPlayerExtensionMode::{OnlinePlayer, SurvivalMode, AI};

/// BoardState stores the internal board representation, as well as other state data that other
/// board components use
///
/// Manually impls PartialEq since SplitSink does not impl PartialEq
#[derive(PartialEq)]
pub struct BoardState {
    pub board_state: Disks,
    pub can_move: bool,
    pub current_player: DiskColor,
    pub game_history: [u8; (BOARD_WIDTH * BOARD_HEIGHT) as usize],
    pub num_moves: u8,
    pub second_player_extension: SecondPlayerExtension,
}

/// Implements functions to check if the game has been won
impl BoardState {
    /// Creates a new empty board, with a callback for rerendering the board.
    pub fn new(rerender_board_callback: Callback<u8>) -> Self {
        BoardState {
            board_state: Disks::default(),
            can_move: true,
            current_player: DiskColor::P1,
            game_history: [0; (BOARD_WIDTH * BOARD_HEIGHT) as usize],
            num_moves: 0,
            second_player_extension: SecondPlayerExtension::new(rerender_board_callback),
        }
    }

    /// Resets the board to how it should be at the start of a new game.
    pub fn reset(&mut self) {
        self.board_state = Disks::default();
        self.can_move = true;
        self.current_player = DiskColor::P1;
        self.game_history = [0; (BOARD_WIDTH * BOARD_HEIGHT) as usize];
        self.num_moves = 0;
        self.second_player_extension.remove_extension();
    }

    /// Does everything required for the next player to make a move in the given column.
    pub fn make_move(&mut self, col: u8) -> Result<(), ()> {
        self.board_state.drop_disk(col)?;
        self.update_can_move_if_won(); // must be called before dropping the disk
        self.update_player();
        self.update_game_history(col);
        Ok(())
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
            if let OnlinePlayer(sender) = self.second_player_extension.get_mode() {
                if let Err(e) = sender.send(selected_col as u8 + col_num_addition) {
                    error!(format!("Failed to send message: {}", e));
                }
            }
        }
    }

    /// If playing singleplayer, give the AI a turn.
    pub fn run_ai_if_applicable(&mut self) {
        if self.second_player_extension.is_ai() || self.second_player_extension.is_survival_mode() {
            if let AI(ai) = self.second_player_extension.get_mode_mut() {
                if self.can_move {
                    let col = ai.get_move(&self.board_state, self.current_player);
                    self.make_move(col).unwrap_or_default();
                }
            } else if let SurvivalMode(ai) = self.second_player_extension.get_mode_mut() {
                if self.can_move {
                    let col = ai.get_move(&self.board_state, self.current_player);
                    self.make_move(col).unwrap_or_default();
                } else {
                    ai.increment_difficulty();
                    self.board_state = Disks::default();
                    self.game_history = [0u8; (BOARD_WIDTH * BOARD_HEIGHT) as usize];
                    self.num_moves = 0;
                    self.can_move = true;
                }
            }
        }
    }

    /// Handles all the board changes based on a message from the second player.
    pub fn update_state_from_second_player_msg(&mut self, mut message: u8) {
        // initialization, telling the client which player they are
        if message == ConnectionProtocol::IS_PLAYER_1 {
            self.current_player = DiskColor::P1;
        } else if message == ConnectionProtocol::IS_PLAYER_2 {
            self.current_player = DiskColor::P2;
            self.can_move = false;
        }
        // if the message communicates a win, extract which column was the winning play
        else if ConnectionProtocol::COL_0 + ConnectionProtocol::WINNING_MOVE_ADDITION <= message
            && message <= ConnectionProtocol::COL_6 + ConnectionProtocol::WINNING_MOVE_ADDITION
        {
            message -= ConnectionProtocol::WINNING_MOVE_ADDITION;
        }
        // if the message is a non-winning move, it will be the client's turn next, so they can move
        else {
            self.can_move = true;
        }

        // if the message is a valid column, make the move
        if ConnectionProtocol::COL_0 <= message && message <= ConnectionProtocol::COL_6 {
            self.board_state.drop_disk(message).unwrap(); // TODO: Handle error
        }
        log!(format!("Received {}", message));
    }

    // Resets the board, and requests a server connection.
    pub fn init_online(&mut self) {
        self.reset(); // reset board data
        self.second_player_extension.init_online(); // set the second player to be online
    }

    // Resets the board, and extends with an AI as the second player.
    pub fn init_ai(&mut self, ai: Box<dyn AITrait>) {
        self.reset(); // reset board data
        self.second_player_extension.init_ai(ai); // set the second player to be an AI
    }

    // Resets the board, and extends to survival mode (second player is AI that improves each round).
    pub fn init_survival(&mut self, starting_ai: Box<dyn SurvivalAI>) {
        self.reset(); // reset board data
        self.second_player_extension.init_survival(starting_ai); // set the second player to be an AI
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
