//! The position lookup table stores calculated board states for given
//! positions so that it doesn't need to recalculate them every time.

/*
 * This file is part of Rust-Connect-Four
 *
 * File derived from Connect4 Game Solver <https://github.com/PascalPons/connect4>
 * Copyright (C) 2017-2019 Pascal Pons <contact@gamesolver.org>
 *
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

use crate::util::disks::Disks;

use std::collections::HashMap;

/// Struct to store previously calculated positions
pub struct PositionLookupTable {
    table: HashMap<u64, i8>,
}

impl PositionLookupTable {
    /// Create a new PositionLookupTable with the specified minimum capacity
    pub fn new(min_capacity: usize) -> Self {
        Self {
            table: HashMap::with_capacity(min_capacity),
        }
    }

    // Insert a position and corresponding score into the table. DOES NOT check if the key already exists.
    pub fn insert(&mut self, position: &Disks, score: i8) {
        self.table.insert(position.get_key(), score);
    }

    // Returns the value associated with the key, or None if the key does not exist.
    pub fn get(&self, position: &Disks) -> Option<i8> {
        self.table.get(&position.get_key()).copied()
    }
}
