use constants::ConnectionProtocol;
use tokio::sync::mpsc::UnboundedSender;
use yew::Callback;

use crate::{
    ai::{ai, perfect::PerfectAI, random::RandomAI},
    util::net::{
        self,
        ServerMessage::{self, BoardState as BoardStateMessage, SimpleMessage, UndoMove},
    },
};

use std::{cell::RefCell, rc::Rc};

pub enum SecondPlayerExtensionMode {
    OnlinePlayer {
        sender: UnboundedSender<ServerMessage>,
        send_update_as_col_num: Rc<RefCell<bool>>,
    }, // vs another person over the internet
    AI(Box<dyn ai::AI>),                   // singleplayer vs bot
    SurvivalMode(Box<dyn ai::SurvivalAI>), // AI mode, but gets progressively harder
    None,                                  // local multiplayer
}

use SecondPlayerExtensionMode::{None, OnlinePlayer, SurvivalMode, AI};

impl PartialEq for SecondPlayerExtensionMode {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (OnlinePlayer { .. }, OnlinePlayer { .. }) => true,
            (AI(_), AI(_)) => true,
            (SurvivalMode(_), SurvivalMode(_)) => true,
            (None, None) => true,
            _ => false,
        }
    }
}

#[derive(PartialEq)]
pub struct SecondPlayerExtension {
    mode: SecondPlayerExtensionMode,
    rerender_board_callback: Callback<ServerMessage>,
}

use super::{
    board_state::BoardState,
    util::{SecondPlayerAIMode, SecondPlayerSurvivalAIMode},
};

impl SecondPlayerExtension {
    pub fn new(rerender_board_callback: Callback<ServerMessage>) -> Self {
        Self {
            mode: None,
            rerender_board_callback,
        }
    }

    pub fn remove_extension(&mut self) {
        self.mode = None;
    }

    /// Discards previous extension, and establishes a connection to the server.
    pub fn init_online(&mut self, lobby: String) {
        self.mode = match net::spawn_connection_tasks(self.rerender_board_callback.clone(), lobby) {
            Ok((sender, send_update_as_col_num)) => OnlinePlayer {
                sender,
                send_update_as_col_num,
            },
            _ => None,
        }
    }

    /// Discards the previous extension, and replaces it with a new AI.
    pub fn init_ai(&mut self, ai_type: SecondPlayerAIMode) {
        self.mode = AI(match ai_type {
            SecondPlayerAIMode::Random => {
                Box::new(RandomAI::new(self.rerender_board_callback.clone()))
            }
            SecondPlayerAIMode::Perfect => {
                Box::new(PerfectAI::new(15, self.rerender_board_callback.clone()))
            }
        });
    }

    // Discards the previous extension, and creates a new survival mode.
    pub fn init_survival(&mut self, ai_type: SecondPlayerSurvivalAIMode) {
        self.mode = SurvivalMode(match ai_type {
            SecondPlayerSurvivalAIMode::Perfect => {
                Box::new(PerfectAI::new(1, self.rerender_board_callback.clone()))
            }
        });
    }

    /// Hands off control to the second player. The board should then wait for
    /// a rerender message with the second player's move.
    /// Returns Result of whether the second player extension will eventually
    /// call back with a move.
    /// Should always be non-blocking.
    pub fn request_move(&self, selected_col: u8, board_state: &BoardState) -> Result<bool, String> {
        match &self.mode {
            OnlinePlayer {
                sender,
                send_update_as_col_num,
            } => {
                Self::update_server(
                    &board_state,
                    &sender,
                    Rc::clone(send_update_as_col_num),
                    selected_col,
                )?;
            }
            AI(ai) => {
                ai.request_move(&board_state.disks);
            }
            SurvivalMode(ai) => {
                ai.request_move(&board_state.disks);
            }
            None => return Ok(false),
        }
        Ok(true)
    }

    pub fn undo_enabled_for_online(&self) -> bool {
        if let OnlinePlayer {
            send_update_as_col_num: no_undo,
            ..
        } = &self.mode
        {
            !*no_undo.borrow()
        } else {
            false
        }
    }

    pub fn is_online_player(&self) -> bool {
        match &self.mode {
            OnlinePlayer { .. } => true,
            _ => false,
        }
    }
    pub fn is_ai(&self) -> bool {
        match &self.mode {
            AI(_) => true,
            _ => false,
        }
    }
    pub fn is_survival_mode(&self) -> bool {
        match &self.mode {
            SurvivalMode(_) => true,
            _ => false,
        }
    }
    pub fn is_none(&self) -> bool {
        match &self.mode {
            SecondPlayerExtensionMode::None => true,
            _ => false,
        }
    }
    pub fn increment_survival_mode_difficulty(&mut self) {
        if let SurvivalMode(ai) = &mut self.mode {
            ai.increment_difficulty();
        }
    }

    fn update_server(
        board_state: &BoardState,
        sender: &UnboundedSender<ServerMessage>,
        send_update_as_col_num: Rc<RefCell<bool>>,
        selected_col: u8,
    ) -> Result<(), String> {
        let update = if *send_update_as_col_num.borrow() {
            SimpleMessage(selected_col)
        } else {
            if selected_col == ConnectionProtocol::UNDO {
                UndoMove(board_state.disks.to_game_update(!board_state.can_move))
            } else {
                BoardStateMessage(board_state.disks.to_game_update(!board_state.can_move))
            }
        };
        if let Err(e) = sender.send(update) {
            return Err(format!("Failed to send message: {}", e));
        }
        Ok(())
    }
}
