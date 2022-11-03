use crate::constants::*;
use crate::util::util::{DiskColor, DiskData, Disks};
use std::cmp::min;

#[derive(PartialEq)]
pub struct BoardState {
    pub board_state: Disks,
    pub current_player: DiskColor,
    pub game_won: bool,
}

impl Default for BoardState {
    fn default() -> Self {
        Self {
            board_state: [[DiskColor::Empty; BOARD_WIDTH]; BOARD_HEIGHT],
            current_player: DiskColor::P1,
            game_won: false,
        }
    }
}

impl BoardState {
    pub fn check_winner(&self, new_disk: DiskData) -> bool {
        if new_disk.row < BOARD_HEIGHT - 3
            && self.board_state[new_disk.row + 1][new_disk.col] == new_disk.color
            && self.board_state[new_disk.row + 2][new_disk.col] == new_disk.color
            && self.board_state[new_disk.row + 3][new_disk.col] == new_disk.color
        {
            return true;
        }

        if Self::check_lateral(&self, &new_disk)
            || Self::check_right_diag(&self, &new_disk)
            || Self::check_left_diag(&self, &new_disk)
        {
            return true;
        }

        false
    }

    fn check_lateral(&self, new_disk: &DiskData) -> bool {
        let mut left_count = 0;
        while left_count < new_disk.left_range {
            if self.board_state[new_disk.row][new_disk.col - 1 - left_count] != new_disk.color {
                break;
            }
            left_count += 1;
        }
        if left_count == 3 {
            return true;
        }

        let mut right_count = 0;
        while right_count < new_disk.right_range {
            if self.board_state[new_disk.row][new_disk.col + 1 + right_count] != new_disk.color {
                break;
            }
            right_count += 1;
            if left_count + right_count == 3 {
                return true;
            }
        }
        false
    }

    fn check_right_diag(&self, new_disk: &DiskData) -> bool {
        let mut top_left_count = 0;
        while top_left_count < min(new_disk.up_range, new_disk.left_range) {
            if self.board_state[new_disk.row - 1 - top_left_count]
                [new_disk.col - 1 - top_left_count]
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
            if self.board_state[new_disk.row + 1 + bottom_right_count]
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

    fn check_left_diag(&self, new_disk: &DiskData) -> bool {
        let mut top_right_count = 0;
        while top_right_count < min(new_disk.up_range, new_disk.right_range) {
            if self.board_state[new_disk.row - 1 - top_right_count]
                [new_disk.col + 1 + top_right_count]
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
            if self.board_state[new_disk.row + 1 + bottom_left_count]
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