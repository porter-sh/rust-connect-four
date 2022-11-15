use crate::util::util::{DiskColor, Disks};

pub trait AI {
    fn get_move(&self, board: &Disks, player: DiskColor) -> u8;
}
