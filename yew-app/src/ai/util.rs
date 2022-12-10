//! Utility functions to be used by multiple AI implementations.

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

use rand::seq::SliceRandom;

pub const AI_INCREMENT_MESSAGE: u8 = 8;
pub const BRUTE_FORCE_SURVIVAL_DIFFICULTY_INCREMENT: u8 = 4;

/// Given a list of columns to choose from, return one at random.
pub fn random_col_from_options(options: &Vec<u8>) -> Option<&u8> {
    options.choose(&mut rand::thread_rng())
}
