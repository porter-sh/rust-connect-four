//! Contains the definition of Disks, which is the internal representation of
//! the game board with bits. This allows for super efficient calculation of if
//! there was a winner, and takes up less memory.

use super::util::DiskColor;
use constants::{GameUpdate, BOARD_HEIGHT, BOARD_WIDTH};

/// 2D array of player disks to internally store the board state
#[derive(PartialEq, Clone, Debug)]
pub struct Disks {
    position: u64, // records the location of disks for the current player as 1s
    mask: u64,     // records the location of all disks as 1s
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

impl From<GameUpdate> for Disks {
    fn from(game: GameUpdate) -> Self {
        Self {
            position: game.position,
            mask: game.mask,
            is_p1_turn: game.is_p1_turn,
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
    pub fn drop_disk(&mut self, col: u8) -> Result<(), String> {
        if self.is_col_full(col) {
            Err("Cannot drop disk in full column".to_string())
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
            num_disks += self.first_opening_in_col(col);
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
        if row > 0 {
            self.mask ^= (1 << row - 1) << (col * (BOARD_HEIGHT + 1));
            self.position ^= self.mask;
            self.is_p1_turn = !self.is_p1_turn;
        }
    }

    /// Returns a unique key representing the current board state,
    /// used for the position lookup table.
    pub fn get_key(&self) -> u64 {
        self.mask + self.position
    }

    pub fn to_game_update(&self, game_won: bool) -> GameUpdate {
        GameUpdate {
            position: self.position,
            mask: self.mask,
            is_p1_turn: self.is_p1_turn,
            game_won,
        }
    }

    pub fn get_is_p1_turn(&self) -> bool {
        self.is_p1_turn
    }

    ///// PRIVATE METHODS /////

    /// Returns the first empty row in the column, or BOARD_HEIGHT if the column is full
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
