use tokio::sync::mpsc::UnboundedSender;
use yew::Callback;

use crate::ai::{ai, perfect::PerfectAI};

pub enum SecondPlayerExtensionMode {
    OnlinePlayer(UnboundedSender<u8>), // vs another person over the internet
    AI(Box<dyn ai::AI>),               // singleplayer vs bot
    SurvivalMode(PerfectAI),           // AI mode, but gets progressively harder
    None,                              // local multiplayer
}

impl PartialEq for SecondPlayerExtensionMode {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::OnlinePlayer(_), Self::OnlinePlayer(_)) => true,
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
    rerender_board_callback: Callback<u8>,
}

use SecondPlayerExtensionMode::{None, OnlinePlayer, SurvivalMode, AI};

impl SecondPlayerExtension {
    pub fn new(rerender_board_callback: Callback<u8>) -> Self {
        Self {
            mode: SecondPlayerExtensionMode::None,
            rerender_board_callback,
        }
    }

    pub fn remove_extension(&mut self) {
        self.mode = SecondPlayerExtensionMode::None;
    }

    /// Discards previous extension, and establishes a connection to the server.
    /// TODO: encapsulate server communication in a separate module
    pub fn init_online(&mut self) {
        todo!();
    }

    /// Discards the previous extension, and replaces it with a new AI.
    pub fn init_ai(&mut self, ai: Box<dyn ai::AI>) {
        self.mode = SecondPlayerExtensionMode::AI(ai);
    }

    // Discards the previous extension, and creates a new survival mode.
    pub fn init_survival(&mut self, starting_ai: Box<dyn ai::AI>) {
        todo!();
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

    pub fn is_online_player(&self) -> bool {
        match &self.mode {
            OnlinePlayer(_) => true,
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
            None => true,
            _ => false,
        }
    }
}
