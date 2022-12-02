//! Contains the PerfectAI implementation.
//! This AI uses futures to asynchronously find the best move using the
//! PerfectAIHelper.

use super::super::{
    ai::{SurvivalAI, AI},
    perfect::PerfectAI,
    perfect_helper::PerfectAIHelper,
    position_lookup_table::PositionLookupTable,
    util::AI_INCREMENT_MESSAGE,
};
use crate::{
    ai::util::PERFECT_SURVIVAL_DIFFICULTY_INCREMENT,
    util::{disks::Disks, util::GameUpdateMessage},
};
use constants::*;
use gloo::console::log;
use tokio::sync::mpsc::{self, UnboundedReceiver};
use wasm_bindgen_futures::spawn_local;
use yew::Callback;

use GameUpdateMessage::SimpleMessage;

impl PerfectAI {
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
            let mut ai = PerfectAIHelper::new(max_moves_look_ahead);
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

impl AI for PerfectAI {
    fn request_move(&self, disks: &Disks) -> u8 {
        self.request_sender
            .send(GameUpdateMessage::Disks(disks.clone()))
            .unwrap_or_default();
        BOARD_WIDTH
    }
}

////////////////////////////////////////////////////////////////////////////////

impl SurvivalAI for PerfectAI {
    /// Used for survival mode, to make the AI harder each round.
    fn increment_difficulty(&mut self) {
        self.request_sender
            .send(SimpleMessage(AI_INCREMENT_MESSAGE))
            .unwrap_or_default();
    }
}
