//! The position lookup table stores calculated board states for given
//! positions so that it doesn't need to recalculate them every time.
use std::collections::HashMap;

use crate::util::disks::Disks;

pub struct PositionLookupTable {
    table: HashMap<u64, i8>,
}

impl PositionLookupTable {
    pub fn new(min_capacity: usize) -> Self {
        Self {
            table: HashMap::with_capacity(min_capacity),
        }
    }

    // Insert a pair into the table. DOES NOT check if the key already exists.
    pub fn insert(&mut self, position: &Disks, score: i8) {
        self.table.insert(position.get_key(), score);
    }

    // Returns the value associated with the key, or None if the key does not exist.
    pub fn get(&self, position: &Disks) -> Option<i8> {
        self.table.get(&position.get_key()).copied()
    }

    // pub fn get_num_entries(&self) -> usize {
    //     self.table.len()
    // }
}
