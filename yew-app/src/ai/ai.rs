//! Contains the AI trait, which defines the interface for the AI implementations.

use crate::util::util::Disks;

pub trait AI {
    /// Requests the next move, possibly asynchronously. The AI should use a previously provided callback to update the board.
    fn request_move(&self, disks: &Disks);
}

pub trait SurvivalAI: AI {
    fn increment_difficulty(&mut self);
}
