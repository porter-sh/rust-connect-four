//! Contains the BruteForceAI struct.
//! This AI asynchronously finds the best move using the
//! BruteForceAIHelper, which looks ahead several moves into the future.

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

use super::{
    super::{
        ai::{SurvivalAI, AI},
        util::AI_INCREMENT_MESSAGE,
    },
    brute_force_helper::BruteForceAIHelper,
    position_lookup_table::PositionLookupTable,
};
use crate::{
    ai::util::BRUTE_FORCE_SURVIVAL_DIFFICULTY_INCREMENT,
    util::{disks::Disks, util::GameUpdateMessage},
};
use constants::*;
use gloo::console::log;
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};
use wasm_bindgen_futures::spawn_local;
use yew::Callback;

use std::{
    cell::RefCell,
    rc::Rc
};

use GameUpdateMessage::SimpleMessage;

/// Struct to asynchronously run the BruteForceAIHelper to find the best possible move
pub struct BruteForceAI {
    pub request_sender: UnboundedSender<GameUpdateMessage>,
    difficulty_level: Rc<RefCell<u8>>
}

impl BruteForceAI {
    /// Creates a BruteForceAI and spawns a computational task to asynchronously run the AI algorithm
    pub fn new(
        max_moves_look_ahead: u8,
        rerender_board_callback: Callback<GameUpdateMessage>,
    ) -> Self {
        let (request_sender, receiver) = mpsc::unbounded_channel();
        let difficulty_level = Rc::new(RefCell::new(1));
        Self::spawn_computation_task(
            receiver,
            rerender_board_callback,
            max_moves_look_ahead,
            Rc::clone(&difficulty_level)
        );
        Self { request_sender, difficulty_level }
    }

    /// Creates a task with a BruteForceAIHelper to run the AI algorithm
    fn spawn_computation_task(
        mut receiver: UnboundedReceiver<GameUpdateMessage>,
        rerender_board_callback: Callback<GameUpdateMessage>,
        max_moves_look_ahead: u8,
        difficulty_level: Rc<RefCell<u8>>
    ) {
        spawn_local(async move {
            let mut ai = BruteForceAIHelper::new(max_moves_look_ahead);
            // Everytime a move is requested from the AI
            while let Some(msg) = receiver.recv().await {
                match msg {
                    GameUpdateMessage::Disks(disks) => {
                        let ai_move = ai.get_move(&disks).await;
                        // if let Err(_) = receiver.try_recv() {
                        rerender_board_callback.emit(SimpleMessage(ai_move)); // Make the move
                        // }
                    }
                    GameUpdateMessage::SimpleMessage(inc) => {
                        // Increase the difficulty
                        if inc == AI_INCREMENT_MESSAGE {
                            *difficulty_level.borrow_mut() += 1;
                            ai.max_moves_look_ahead += BRUTE_FORCE_SURVIVAL_DIFFICULTY_INCREMENT;
                            // Old table may contain invalid scores, now that the AI looks further ahead into the future
                            ai.position_lookup_table = PositionLookupTable::new(LOOKUP_TABLE_SIZE);
                            log!(format!(
                                "Difficulty increased to {}",
                                ai.max_moves_look_ahead
                            ));
                        }
                    }
                    _ => panic!("Received message the AI thread cannot handle."),
                }
            }
        });
    }
}

impl AI for BruteForceAI {
    /// Give the current disk arrangement to the computational task to find the next move for the AI to make
    fn request_move(&self, disks: &Disks) -> u8 {
        self.request_sender
            .send(GameUpdateMessage::Disks(disks.clone()))
            .unwrap_or_default();
        BOARD_WIDTH
    }
}

impl SurvivalAI for BruteForceAI {
    /// Used for survival mode, to make the AI harder each round.
    fn increment_difficulty(&mut self) {
        // Tell the helper on the computational task to increase the difficulty
        self.request_sender
            .send(SimpleMessage(AI_INCREMENT_MESSAGE))
            .unwrap_or_default();
    }
    fn get_difficulty_level(&self) -> u8 {
        *self.difficulty_level.borrow()
    }
}
