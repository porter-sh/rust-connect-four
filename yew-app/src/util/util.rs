//! util contains helper structs for the player disks

use crate::ai::ai;
use constants::*;
use std::cmp::min;

use tokio::sync::mpsc::UnboundedSender;

/// 2D array of player disks to internally store the board state

#[derive(PartialEq, Clone)]
pub struct Disks {
    position: u64,
    mask: u64,
    is_p1_turn: bool,
}

impl Default for Disks {
    fn default() -> Self {
        Self {
            position: 0,
            mask: 0,
            is_p1_turn: true,
        }
    }
}

/// Manages the data about the disks. Manages everything through an interface
/// for calculation of who won, which columns are full, empty, how many disks
/// there are etc. This way it is easy for us to make optimizations later without
/// needing to change how the interface is used.
impl Disks {
    /// Returns if the other player has won
    pub fn check_last_drop_won(&self) -> bool {
        let other_player_position = self.position ^ self.mask;
        // check horizontal wins
        let mut temp: u64 = other_player_position & (other_player_position >> (BOARD_HEIGHT + 1));
        if (temp & (temp >> (2 * (BOARD_HEIGHT + 1)))) != 0 {
            return true;
        }

        // check \ diagonal wins
        temp = other_player_position & (other_player_position >> BOARD_HEIGHT);
        if (temp & (temp >> (2 * BOARD_HEIGHT))) != 0 {
            return true;
        }

        // check / diagonal wins
        temp = other_player_position & (other_player_position >> (BOARD_HEIGHT + 2));
        if (temp & (temp >> (2 * (BOARD_HEIGHT + 2)))) != 0 {
            return true;
        }

        // check vertical wins
        temp = other_player_position & (other_player_position >> 1);
        if (temp & (temp >> 2)) != 0 {
            return true;
        }

        return false;
    }

    /// Puts a disk of the given color in the given column
    pub fn drop_disk(&mut self, col: u8) -> Result<(), ()> {
        if self.is_col_full(col) {
            Err(())
        } else {
            self.is_p1_turn = !self.is_p1_turn;
            self.position ^= self.mask;
            self.mask |= self.mask + (1 << (col * (BOARD_HEIGHT + 1)));
            Ok(())
        }
    }

    /// Returns the color of the disk at the given location
    pub fn get_disk(&self, row: u8, col: u8) -> DiskColor {
        let bit = self.mask & ((1 << row) << (col * (BOARD_HEIGHT + 1)));
        if bit == 0 {
            return DiskColor::Empty;
        }
        return if ((self.position & bit) != 0) == self.is_p1_turn {
            DiskColor::P1
        } else {
            DiskColor::P2
        };
    }

    /// Returns the total number of disks on the board
    pub fn get_num_disks(&self) -> u8 {
        let mut num_disks = 0u8;
        for col in 0..(BOARD_WIDTH as u8) {
            num_disks += (BOARD_HEIGHT as u8) - self.first_opening_in_col(col);
        }
        num_disks
    }

    /// Returns whether the given column is is full (has no open slots)
    pub fn is_col_full(&self, col: u8) -> bool {
        self.mask & ((1 << BOARD_HEIGHT - 1) << (col * (BOARD_HEIGHT + 1))) != 0
    }

    /// Returns whether the entire board is full (has no open slots)
    pub fn is_full(&self) -> bool {
        for col in 0..(BOARD_WIDTH as u8) {
            if !self.is_col_full(col) {
                return false;
            }
        }
        true
    }

    /// Gets the number of columns that are not full
    pub fn num_open_cols(&self) -> u8 {
        let mut num_open_cols = 0u8;
        for col in 0..(BOARD_WIDTH as u8) {
            if !self.is_col_full(col) {
                num_open_cols += 1;
            }
        }
        num_open_cols
    }

    /// Takes the top disk off the given column
    pub fn rm_disk_from_col(&mut self, col: u8) {
        let row = self.first_opening_in_col(col);
        self.mask ^= (1 << row - 1) << (col * (BOARD_HEIGHT + 1));
        self.position ^= self.mask;
        self.is_p1_turn = !self.is_p1_turn;
    }

    ///// PRIVATE METHODS /////

    /// Returns the first empty row in the column, or 0 if the column is full
    fn first_opening_in_col(&self, col: u8) -> u8 {
        let mut idx = 1 << (col * (BOARD_HEIGHT + 1));
        for row in 0..BOARD_HEIGHT {
            if self.mask & idx == 0 {
                return row;
            }
            idx <<= 1;
        }
        BOARD_HEIGHT
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

/// Enum to augment the game with either a server connection, or an AI,
/// depending on what game mode is selected.
pub enum SecondPlayerExtension {
    OnlinePlayer(UnboundedSender<u8>), // channel to send column selection to the server
    AI(Box<dyn ai::AI>),               // AI for singleplayer
    None,                              // local multiplayer
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
