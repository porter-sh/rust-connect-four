//! util contains helper structs for the player disks

/*
 * This file is part of Rust-Connect-Four
 * Copyright (C) 2022 Alexander Broihier <alexanderbroihier@gmail.com>
 * Copyright (C) 2022 Porter Shawver <portershawver@gmail.com>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published
 * by the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

use crate::{ai::ai, util::disks::Disks};
use constants::*;
use std::{cell::RefCell, rc::Rc};
use tokio::sync::mpsc::UnboundedSender;

/// Enum to store the state at a particular board space
/// Either Empty or the corresponding player who has a disk in that spot
/// Can also be used to track which player a client is
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum DiskColor {
    Empty,
    P1,
    P2,
}

impl DiskColor {
    /// Switches from P1 to P2 and vice versa
    pub fn opposite(&self) -> DiskColor {
        match self {
            DiskColor::Empty => DiskColor::Empty, // spectating
            DiskColor::P1 => DiskColor::P2,
            DiskColor::P2 => DiskColor::P1,
        }
    }
}

/// Enum that represents a message for the SecondPlayerExtension
#[derive(Debug)]
pub enum GameUpdateMessage {
    BoardState(GameUpdate),
    Disks(Disks),
    SimpleMessage(u8),
    UndoMove(GameUpdate),
}

/// Enum that represents the result of a move requested by the second player extension
#[derive(PartialEq)]
pub enum RequestMoveResult {
    WillRerenderLater,
    RerenderNow(u8),
    NoRequestMade,
}

/// Enum that represents an AI implementation to use
pub enum SecondPlayerAIMode {
    Random,
    BruteForce,
}

/// Enum that represents a SurvivalAI implementation to use
pub enum SecondPlayerSurvivalAIMode {
    BruteForce,
}

/// Enum to store different kinds of second players
pub enum SecondPlayerExtensionMode {
    OnlinePlayer {
        sender: UnboundedSender<GameUpdateMessage>,
        send_update_as_col_num: Rc<RefCell<bool>>,
    }, // vs another person over the internet
    AI {
        ai: Box<dyn ai::AI>,
        ai_color: DiskColor,
    }, // singleplayer vs bot
    SurvivalMode {
        ai: Box<dyn ai::SurvivalAI>,
        ai_color: DiskColor,
    }, // AI mode, but gets progressively harder
    None, // local multiplayer
}

use SecondPlayerExtensionMode::{None, OnlinePlayer, SurvivalMode, AI};

/// SecondPlayerExtensionModes of the same variant are treated as equal
impl PartialEq for SecondPlayerExtensionMode {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (OnlinePlayer { .. }, OnlinePlayer { .. }) => true,
            (AI { .. }, AI { .. }) => true,
            (SurvivalMode { .. }, SurvivalMode { .. }) => true,
            (None, None) => true,
            _ => false,
        }
    }
}
