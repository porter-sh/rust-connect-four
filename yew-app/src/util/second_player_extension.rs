use tokio::sync::mpsc::UnboundedSender;
use yew::Callback;

use crate::{
    ai::ai,
    util::net::{self, ServerMessage}
};

use std::{cell::RefCell, rc::Rc};

pub enum SecondPlayerExtensionMode {
    OnlinePlayer {
        sender: UnboundedSender<ServerMessage>,
        send_update_as_col_num: Rc<RefCell<bool>>
    },                                               // vs another person over the internet
    AI(Box<dyn ai::AI>),                             // singleplayer vs bot
    SurvivalMode(Box<dyn ai::SurvivalAI>),           // AI mode, but gets progressively harder
    None,                                            // local multiplayer
}

impl PartialEq for SecondPlayerExtensionMode {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::OnlinePlayer {..}, Self::OnlinePlayer {..}) => true,
            (Self::AI(_), Self::AI(_)) => true,
            (Self::SurvivalMode(_), Self::SurvivalMode(_)) => true,
            (Self::None, Self::None) => true,
            _ => false,
        }
    }
}

#[derive(PartialEq)]
pub struct SecondPlayerExtension {
    mode: SecondPlayerExtensionMode,
    rerender_board_callback: Callback<ServerMessage>,
}

use SecondPlayerExtensionMode::{None, AI, OnlinePlayer, SurvivalMode};

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
    /// TODO: encapsulate server communication in a separate module
    pub fn init_online(&mut self, lobby: String) {
        self.mode = match net::spawn_connection_tasks(self.rerender_board_callback.clone(), lobby) {
            Ok((sender, send_update_as_col_num)) => OnlinePlayer { sender, send_update_as_col_num },
            _ => None
        }
    }

    /// Discards the previous extension, and replaces it with a new AI.
    pub fn init_ai(&mut self, ai: Box<dyn ai::AI>) {
        self.mode = AI(ai);
    }

    // Discards the previous extension, and creates a new survival mode.
    pub fn init_survival(&mut self, ai: Box<dyn ai::SurvivalAI>) {
        self.mode = SurvivalMode(ai)
    }

    /// Hands off control to the second player. The board should then wait for
    /// a rerender message with the second player's move.
    /// Should always be non-blocking.
    pub fn request_move() {
        todo!();
    }

    pub fn get_mode(&self) -> &SecondPlayerExtensionMode {
        &self.mode
    }

    pub fn get_mode_mut(&mut self) -> &mut SecondPlayerExtensionMode {
        &mut self.mode
    }

    pub fn undo_enabled_for_online(&self) -> bool {
        if let OnlinePlayer {send_update_as_col_num: no_undo, ..} = &self.mode {
            !*no_undo.borrow()
        } else {
            false
        }
    }

    pub fn is_online_player(&self) -> bool {
        match &self.mode {
            OnlinePlayer {..} => true,
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
}
