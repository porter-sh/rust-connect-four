use crate::constants::*;
use std::cmp::min;

pub type Disks = [[DiskColor; BOARD_WIDTH]; BOARD_HEIGHT];

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
