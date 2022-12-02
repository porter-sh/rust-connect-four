//! util contains helper structs for the player disks

use crate::{ai::ai, util::disks::Disks};
use constants::*;
use std::{cell::RefCell, cmp::min, rc::Rc};
use tokio::sync::mpsc::UnboundedSender;

/// Enum to store the state at a particular board space
/// Either Empty or the corresponding player who has a disk in that spot
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum DiskColor {
    Empty,
    P1,
    P2,
}

impl DiskColor {
    pub fn to_str(&self) -> &str {
        match self {
            DiskColor::Empty => "empty",
            DiskColor::P1 => "p1",
            DiskColor::P2 => "p2",
        }
    }
}

/// DiskData contains fields that help determine looping over the board to determine if a dropped disk wins the game
pub struct DiskData {
    pub row: u8,
    pub col: u8,
    pub color: DiskColor,
    pub left_range: u8,
    pub right_range: u8,
    pub up_range: u8,
    pub down_range: u8,
}

impl DiskData {
    /// Create DiskData and store how far in each direction we should loop
    pub fn new(row: u8, col: u8, color: DiskColor) -> Self {
        Self {
            row,
            col,
            color,
            left_range: min(3, col),
            right_range: min(3, BOARD_WIDTH as u8 - col - 1),
            up_range: min(3, row),
            down_range: min(3, BOARD_HEIGHT as u8 - row - 1),
        }
    }
}

/// Enum that represents a message to be sent to the server
#[derive(Debug)]
pub enum GameUpdateMessage {
    BoardState(GameUpdate),
    Disks(Disks),
    SimpleMessage(u8),
    UndoMove(GameUpdate),
}

pub enum SecondPlayerAIMode {
    Random,
    Perfect,
}

pub enum SecondPlayerSurvivalAIMode {
    Perfect,
}

pub enum SecondPlayerExtensionMode {
    OnlinePlayer {
        sender: UnboundedSender<GameUpdateMessage>,
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
