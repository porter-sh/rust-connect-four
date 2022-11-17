use crate::util::util::{DiskColor, Disks};

pub trait AI {
    /// Gets the next move the AI wants to do.
    fn get_move(&mut self, board: &Disks, player: DiskColor) -> u8;
}
