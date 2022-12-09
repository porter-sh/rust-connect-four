//! Contains the PerfectAI struct.
//! This AI uses futures to asynchronously find the best move using the
//! PerfectAIHelper, which looks ahead several moves into the future.

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
    ai::util::PERFECT_SURVIVAL_DIFFICULTY_INCREMENT,
    util::{disks::Disks, util::GameUpdateMessage},
};
use constants::*;
use gloo::console::log;
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};
use wasm_bindgen_futures::spawn_local;
use yew::Callback;

use GameUpdateMessage::SimpleMessage;

pub struct BruteForceAI {
    pub request_sender: UnboundedSender<GameUpdateMessage>,
}

impl BruteForceAI {
    pub fn new(
        max_moves_look_ahead: u8,
        rerender_board_callback: Callback<GameUpdateMessage>,
    ) -> Self {
        let (request_sender, receiver) = mpsc::unbounded_channel();
        Self::spawn_computation_task(receiver, rerender_board_callback, max_moves_look_ahead);
        Self { request_sender }
    }

    fn spawn_computation_task(
        mut receiver: UnboundedReceiver<GameUpdateMessage>,
        rerender_board_callback: Callback<GameUpdateMessage>,
        max_moves_look_ahead: u8,
    ) {
        spawn_local(async move {
            let mut ai = BruteForceAIHelper::new(max_moves_look_ahead);
            while let Some(msg) = receiver.recv().await {
                match msg {
                    GameUpdateMessage::Disks(disks) => {
                        let ai_move = ai.get_move(&disks).await;
                        // Make sure updated input has not been sent to this receiver
                        // if let Err(_) = receiver.try_recv() { // TODO: comment back in if/when AI properly runs in the background
                        rerender_board_callback.emit(SimpleMessage(ai_move));
                        // }
                    }
                    GameUpdateMessage::SimpleMessage(inc) => {
                        if inc == AI_INCREMENT_MESSAGE {
                            ai.max_moves_look_ahead += PERFECT_SURVIVAL_DIFFICULTY_INCREMENT;
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

////////////////////////////////////////////////////////////////////////////////

impl AI for BruteForceAI {
    fn request_move(&self, disks: &Disks) -> u8 {
        self.request_sender
            .send(GameUpdateMessage::Disks(disks.clone()))
            .unwrap_or_default();
        BOARD_WIDTH
    }
}

////////////////////////////////////////////////////////////////////////////////

impl SurvivalAI for BruteForceAI {
    /// Used for survival mode, to make the AI harder each round.
    fn increment_difficulty(&mut self) {
        self.request_sender
            .send(SimpleMessage(AI_INCREMENT_MESSAGE))
            .unwrap_or_default();
    }
}