//! util contains helper structs for the player disks

use crate::ai::ai;
use constants::*;
use std::cmp::min;

use tokio::sync::mpsc::UnboundedSender;

/// 2D array of player disks to internally store the board state
// pub type Disks1 = [[DiskColor; BOARD_WIDTH]; BOARD_HEIGHT];

#[derive(PartialEq)]
pub struct Disks {
    position: [[DiskColor; BOARD_WIDTH]; BOARD_HEIGHT],
}

impl Default for Disks {
    fn default() -> Self {
        Self {
            position: [[DiskColor::Empty; BOARD_WIDTH]; BOARD_HEIGHT],
        }
    }
}

/// Implements functions to check if the game has been won
impl Disks {
    /// Returns if the game is won by dropping a disk in the location stored by DiskData
    pub fn check_winner(&mut self, col: u8, color: &DiskColor) -> bool {
        let row = self.first_opening_in_col(col);
        // check for a vertical win
        if (row as usize) < BOARD_HEIGHT - 3
            && self.position[(row + 1) as usize][col as usize] == *color
            && self.position[(row + 2) as usize][col as usize] == *color
            && self.position[(row + 3) as usize][col as usize] == *color
        {
            return true;
        }
        // data structure to hold useful information about the new piece
        let new_disk = DiskData::new(row, col, *color);
        // check for a win in other directions
        if Self::check_lateral(&self, &new_disk)
            || Self::check_right_diag(&self, &new_disk)
            || Self::check_left_diag(&self, &new_disk)
        {
            return true;
        }
        // no win found
        false
    }

    pub fn drop_disk(&mut self, col: u8, color: &DiskColor) -> Result<(), ()> {
        let col = col as usize;
        for row in (0..BOARD_HEIGHT).rev() {
            if self.position[row][col] == DiskColor::Empty {
                self.position[row][col] = *color;
                return Ok(());
            }
        }
        Err(())
    }

    pub fn get_disk(&self, row: u8, col: u8) -> DiskColor {
        self.position[row as usize][col as usize]
    }

    pub fn get_num_disks(&self) -> u8 {
        let mut num_disks = 0u8;
        for col in 0..(BOARD_WIDTH as u8) {
            num_disks += (BOARD_HEIGHT as u8) - self.first_opening_in_col(col);
        }
        num_disks
    }

    pub fn is_full(&self, col: u8) -> bool {
        self.first_opening_in_col(col) == 0 as u8
    }

    /// Returns the first empty row in the column, or 0 if the column is full
    fn first_opening_in_col(&self, col: u8) -> u8 {
        for row in (0..(BOARD_HEIGHT as u8)).rev() {
            if self.position[row as usize][col as usize] == DiskColor::Empty {
                return row;
            }
        }
        0
    }

    /// Helper function to check_winner
    /// Returns whether a horizontal win occured
    fn check_lateral(&self, new_disk: &DiskData) -> bool {
        let mut left_count = 0;
        while left_count < new_disk.left_range {
            if self.position[new_disk.row as usize][(new_disk.col - 1 - left_count) as usize]
                != new_disk.color
            {
                break;
            }
            left_count += 1;
        }
        if left_count == 3 {
            return true;
        }

        let mut right_count = 0;
        while right_count < new_disk.right_range {
            if self.position[new_disk.row as usize][(new_disk.col + 1 + right_count) as usize]
                != new_disk.color
            {
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
            if self.position[(new_disk.row - 1 - top_left_count) as usize]
                [(new_disk.col - 1 - top_left_count) as usize]
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
            if self.position[(new_disk.row + 1 + bottom_right_count) as usize]
                [(new_disk.col + 1 + bottom_right_count) as usize]
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
            if self.position[(new_disk.row - 1 - top_right_count) as usize]
                [(new_disk.col + 1 + top_right_count) as usize]
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
            if self.position[(new_disk.row + 1 + bottom_left_count) as usize]
                [(new_disk.col - 1 - bottom_left_count) as usize]
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
    pub row: u8,
    pub col: u8,
    pub color: DiskColor,
    pub left_range: u8,
    pub right_range: u8,
    pub up_range: u8,
    pub down_range: u8,
}

impl DiskData {
    /// Create DiskData and store how far in each direction we should loop
    pub fn new(row: u8, col: u8, color: DiskColor) -> Self {
        Self {
            row,
            col,
            color,
            left_range: min(3, col),
            right_range: min(3, BOARD_WIDTH as u8 - col - 1),
            up_range: min(3, row),
            down_range: min(3, BOARD_HEIGHT as u8 - row - 1),
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
