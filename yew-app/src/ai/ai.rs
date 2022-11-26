//! Contains the AI trait, which defines the interface for the AI implementations.
use yew::Callback;

use crate::util::util::{DiskColor, Disks};

pub trait AI {
    /// Gets the next move the AI wants to do.
    fn get_move(&mut self, board: &Disks, player: DiskColor) -> u8;

    /// Requests the next move asynchronously. The AI should use the callback to update the board.
    fn request_move(&mut self, callback: Callback<u8>);
}

pub trait SurvivalAI: AI {
    fn increment_difficulty(&mut self);
}
