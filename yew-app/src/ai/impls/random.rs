//! Contains the RandomAI struct.
//! RandomAI randomly chooses any open column.

/*
 * This file is part of Rust-Connect-Four
 * Copyright (C) 2022 Alexander Broihier <alexanderbroihier@gmail.com>
 * Copyright (C) 2022 Porter Shawver <portershawver@gmail.com>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published
 * by the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

use crate::{
    ai::{ai::AI, util},
    util::disks::Disks,
};
use constants::BOARD_WIDTH;

pub struct RandomAI;

impl AI for RandomAI {
    /// Gets a random move from the available columns.
    fn request_move(&self, disks: &Disks) -> u8 {
        // find which columns are open
        let mut available_cols = Vec::with_capacity(BOARD_WIDTH as usize);
        for col in 0..(BOARD_WIDTH as u8) {
            if !disks.is_col_full(col) {
                available_cols.push(col);
            }
        }
        // chose one of the available columns at random
        match util::random_col_from_options(&available_cols) {
            Some(col) => {
                return *col;
            }
            _ => panic!("Fix the random AI, should have returned a value"),
        }
    }
}
