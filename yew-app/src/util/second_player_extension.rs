//! second_player_extension contains the SecondPlayerExtension struct, which stores
//! special second player frameworks and can request a move from a non local player

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

use constants::*;
use tokio::sync::mpsc::UnboundedSender;
use yew::Callback;

use super::{
    board_state::BoardState,
    util::{DiskColor, SecondPlayerAIMode, SecondPlayerSurvivalAIMode},
};
use crate::{
    ai::impls::{brute_force::BruteForceAI, random::RandomAI},
    util::{
        net,
        util::{
            GameUpdateMessage::{self, BoardState as BoardStateMessage, SimpleMessage, UndoMove},
            RequestMoveResult, SecondPlayerExtensionMode,
        },
    },
};

use SecondPlayerExtensionMode::{None, OnlinePlayer, SurvivalMode, AI};

use std::{cell::RefCell, rc::Rc};

/// SecondPlayerExtension stores second player frameworks and can request moves from the second player
#[derive(PartialEq)]
pub struct SecondPlayerExtension {
    pub mode: SecondPlayerExtensionMode,
    rerender_board_callback: Callback<GameUpdateMessage>,
}

impl SecondPlayerExtension {
    /// Create an empty (used for local multiplayer) SecondPlayerExtension
    pub fn new(rerender_board_callback: Callback<GameUpdateMessage>) -> Self {
        Self {
            mode: None,
            rerender_board_callback,
        }
    }

    /// Remove any second player framework (makes it suitable for local multiplayer)
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
            _ => None, // connection failed
        }
    }

    /// Discards the previous extension, and replaces it with a new AI.
    pub fn init_ai(&mut self, ai_type: SecondPlayerAIMode) {
        self.mode = AI {
            ai: match ai_type {
                SecondPlayerAIMode::Random => Box::new(RandomAI),
                SecondPlayerAIMode::BruteForce => {
                    Box::new(BruteForceAI::new(20, self.rerender_board_callback.clone()))
                }
            },
            ai_color: DiskColor::P2,
        };
    }

    // Discards the previous extension, and creates a new survival mode AI.
    pub fn init_survival(&mut self, ai_type: SecondPlayerSurvivalAIMode) {
        self.mode = SurvivalMode {
            ai: match ai_type {
                SecondPlayerSurvivalAIMode::BruteForce => {
                    Box::new(BruteForceAI::new(1, self.rerender_board_callback.clone()))
                }
            },
            ai_color: DiskColor::P2,
        };
    }

    /// Hands off control to the second player. The board should then wait for
    /// a rerender message with the second player's move.
    /// Returns Result of whether the second player extension was successfully updated,
    /// and how the second player will eventually return a move.
    pub fn request_move(
        &self,
        selected_col: u8,
        board_state: &BoardState,
    ) -> Result<RequestMoveResult, String> {
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
                )?; // Send the server the updated board state / pass off to online opponent
            }
            AI { ai, .. } => {
                if selected_col != ConnectionProtocol::UNDO && board_state.can_move { // Don't run AI if a move was undone
                    let res = ai.request_move(&board_state.disks);
                    return Ok(if res < BOARD_WIDTH {
                        RequestMoveResult::RerenderNow(res) // Propogate up a valid move
                    } else {
                        RequestMoveResult::WillRerenderLater // No valid move yet but request for move made
                    });
                }
            }
            SurvivalMode { ai, .. } => {
                if selected_col != ConnectionProtocol::UNDO && board_state.can_move { // Don't run AI if a move was undone
                    let res = ai.request_move(&board_state.disks);
                    return Ok(if res < BOARD_WIDTH {
                        RequestMoveResult::RerenderNow(res) // Propogate up a valid move
                    } else {
                        RequestMoveResult::WillRerenderLater // No valid move yet but request for move made
                    });
                }
            }
            None => return Ok(RequestMoveResult::NoRequestMade), // No second player framework (local multiplayer)
        }
        Ok(RequestMoveResult::WillRerenderLater) // Request for move made
    }

    /// Returns whether the SecondPlayerExtension is an online player and undo is enabled for this match
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

    /// Returns whether the SecondPlayerExtension is an online player
    pub fn is_online_player(&self) -> bool {
        match &self.mode {
            OnlinePlayer { .. } => true,
            _ => false,
        }
    }
    /// Returns whether the SecondPlayerExtension is an AI player
    pub fn is_ai(&self) -> bool {
        match &self.mode {
            AI { .. } => true,
            _ => false,
        }
    }
    /// Returns whether the SecondPlayerExtension is a Survival AI player
    pub fn is_survival_mode(&self) -> bool {
        match &self.mode {
            SurvivalMode { .. } => true,
            _ => false,
        }
    }
    /// Returns whether the SecondPlayerExtension is empty (suitable for local multiplayer)
    pub fn is_none(&self) -> bool {
        match &self.mode {
            SecondPlayerExtensionMode::None => true,
            _ => false,
        }
    }
    /// Increments the SurvivalAI difficulty
    /// Does nothing if the SecondPlayerExtension is not a SurvivalMode AI
    pub fn increment_survival_mode_difficulty(&mut self) {
        if let SurvivalMode { ai, .. } = &mut self.mode {
            ai.increment_difficulty();
        }
    }
    /// Gets the difficulty of the SurvivalAI
    /// Returns None if the SecondPlayerExtension is not a SurvivalMode AI
    pub fn get_survival_mode_difficulty(&self) -> Option<u8> {
        if let SurvivalMode { ai, .. } = &self.mode {
            Some(ai.get_difficulty_level())
        } else {
            Option::None
        }
    }
    /// If the SecondPlayerExtension is an AI or SurvivalAI,
    /// switches the AI from P1 to P2 or vice versa
    pub fn switch_ai_color_if_ai_or_survival(&mut self, undo_color: DiskColor) {
        if let AI { ai_color, .. } = &mut self.mode {
            if *ai_color == undo_color {
                *ai_color = ai_color.opposite();
            }
        } else if let SurvivalMode { ai_color, .. } = &mut self.mode {
            if *ai_color == undo_color {
                *ai_color = ai_color.opposite();
            }
        }
    }

    /// Sends a message to the server writer task
    fn update_server(
        board_state: &BoardState,
        sender: &UnboundedSender<GameUpdateMessage>,
        send_update_as_col_num: Rc<RefCell<bool>>,
        selected_col: u8,
    ) -> Result<(), String> {
        // Get the update in the appropriate format
        let update = if *send_update_as_col_num.borrow() {
            SimpleMessage(selected_col)
        } else {
            let update = board_state.disks.to_game_update(!board_state.can_move);
            if selected_col == ConnectionProtocol::UNDO {
                UndoMove(update)
            } else {
                BoardStateMessage(update)
            }
        };
        if let Err(e) = sender.send(update) {
            // Failed to send to the server writer thread
            return Err(format!("Failed to send message: {}", e));
        }
        Ok(())
    }
}
