//! util contains helper structs for the player disks

use crate::ai::ai;
use constants::*;
use std::cmp::min;

use tokio::sync::mpsc::UnboundedSender;

/// 2D array of player disks to internally store the board state
// pub type Disks1 = [[DiskColor; BOARD_WIDTH]; BOARD_HEIGHT];

pub struct Disks {
    position: [[DiskColor; BOARD_WIDTH]; BOARD_HEIGHT],
    pub can_move: bool,
}

impl Default for Disks {
    fn default() -> Self {
        Self {
            position: [[DiskColor::Empty; BOARD_WIDTH]; BOARD_HEIGHT],
            can_move: true,
        }
    }
}

/// Implements functions to check if the game has been won
impl Disks {
    /// Returns if the game is won by dropping a disk in the location stored by DiskData
    pub fn check_winner(&mut self, new_disk: DiskData) {
        // check for a vertical win
        if new_disk.row < BOARD_HEIGHT - 3
            && self.position[new_disk.row + 1][new_disk.col] == new_disk.color
            && self.position[new_disk.row + 2][new_disk.col] == new_disk.color
            && self.position[new_disk.row + 3][new_disk.col] == new_disk.color
        {
            self.can_move = false;
        }
        // check for a win in other directions
        else if Self::check_lateral(&self, &new_disk)
            || Self::check_right_diag(&self, &new_disk)
            || Self::check_left_diag(&self, &new_disk)
        {
            self.can_move = false;
        }
    }

    pub fn drop_disk(&mut self, col: u8, color: DiskColor) -> Result<(), ()> {
        let col = col as usize;
        for row in (0..BOARD_HEIGHT).rev() {
            if self.position[row][col] == DiskColor::Empty {
                self.position[row][col] = color;
                return Ok(());
            }
        }
        Err(())
    }

    /// Helper function to check_winner
    /// Returns whether a horizontal win occured
    fn check_lateral(&self, new_disk: &DiskData) -> bool {
        let mut left_count = 0;
        while left_count < new_disk.left_range {
            if self.position[new_disk.row][new_disk.col - 1 - left_count] != new_disk.color {
                break;
            }
            left_count += 1;
        }
        if left_count == 3 {
            return true;
        }

        let mut right_count = 0;
        while right_count < new_disk.right_range {
            if self.position[new_disk.row][new_disk.col + 1 + right_count] != new_disk.color {
                break;
            }
            right_count += 1;
            if left_count + right_count == 3 {
                return true;
            }
        }
        false
    }

    /// Helper function to check_winner
    /// Returns whether a right diagonal win occured
    fn check_right_diag(&self, new_disk: &DiskData) -> bool {
        let mut top_left_count = 0;
        while top_left_count < min(new_disk.up_range, new_disk.left_range) {
            if self.position[new_disk.row - 1 - top_left_count][new_disk.col - 1 - top_left_count]
                != new_disk.color
            {
                break;
            }
            top_left_count += 1;
        }
        if top_left_count == 3 {
            return true;
        }

        let mut bottom_right_count = 0;
        while bottom_right_count < min(new_disk.down_range, new_disk.right_range) {
            if self.position[new_disk.row + 1 + bottom_right_count]
                [new_disk.col + 1 + bottom_right_count]
                != new_disk.color
            {
                break;
            }
            bottom_right_count += 1;
            if top_left_count + bottom_right_count == 3 {
                return true;
            }
        }
        false
    }

    /// Helper function to check_winner
    /// Returns whether a left diagonal win occured
    fn check_left_diag(&self, new_disk: &DiskData) -> bool {
        let mut top_right_count = 0;
        while top_right_count < min(new_disk.up_range, new_disk.right_range) {
            if self.position[new_disk.row - 1 - top_right_count][new_disk.col + 1 + top_right_count]
                != new_disk.color
            {
                break;
            }
            top_right_count += 1;
        }
        if top_right_count == 3 {
            return true;
        }

        let mut bottom_left_count = 0;
        while bottom_left_count < min(new_disk.down_range, new_disk.left_range) {
            if self.position[new_disk.row + 1 + bottom_left_count]
                [new_disk.col - 1 - bottom_left_count]
                != new_disk.color
            {
                break;
            }
            bottom_left_count += 1;
            if top_right_count + bottom_left_count == 3 {
                return true;
            }
        }
        false
    }
}

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

pub enum SecondPlayerExtension {
    OnlinePlayer(UnboundedSender<u8>),
    AI(Box<dyn ai::AI>),
    None,
}

use SecondPlayerExtension::{None, OnlinePlayer, AI};

impl SecondPlayerExtension {
    pub fn is_online_player(&self) -> bool {
        match self {
            OnlinePlayer(_) => true,
            _ => false,
        }
    }
    pub fn is_ai(&self) -> bool {
        match self {
            AI(_) => true,
            _ => false,
        }
    }
    pub fn is_none(&self) -> bool {
        match self {
            None => true,
            _ => false,
        }
    }
}
