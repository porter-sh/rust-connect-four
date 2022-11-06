//! util contains helper structs for the player disks

use crate::constants::*;
use std::cmp::min;

/// 2D array of player disks to internally store the board state
pub type Disks = [[DiskColor; BOARD_WIDTH]; BOARD_HEIGHT];

/// DiskData contains fields that help determine looping over the board to determine if a dropped disk wins the game
pub struct DiskData {
    pub row: usize,
    pub col: usize,
    pub color: DiskColor,
    pub left_range: usize,
    pub right_range: usize,
    pub up_range: usize,
    pub down_range: usize,
}

impl DiskData {
    /// Create DiskData and store how far in each direction we should loop
    pub fn new(row: usize, col: usize, color: DiskColor) -> Self {
        Self {
            row,
            col,
            color,
            left_range: min(3, col),
            right_range: min(3, BOARD_WIDTH - col - 1),
            up_range: min(3, row),
            down_range: min(3, BOARD_HEIGHT - row - 1),
        }
    }
}

/// Enum to store the state at a particular board space
/// Either Empty or the corresponding player who has a disk in that spot
#[derive(Clone, Copy, PartialEq)]
pub enum DiskColor {
    Empty,
    P1,
    P2,
}

impl DiskColor {
    pub fn to_str(&self) -> &str {
        match self {
            DiskColor::Empty => "empty",
            DiskColor::P1 => "p1",
            DiskColor::P2 => "p2",
        }
    }
}
