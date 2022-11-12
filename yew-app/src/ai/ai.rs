use crate::util::util::Disks;

pub trait AI {
    fn get_move(&self, board: &Disks) -> usize;
}
