use std::collections::HashMap;

use crate::util::util::Disks;

struct PositionLookupTable {
    table: HashMap<u64, u8>,
}

impl PositionLookupTable {
    pub fn new() -> Self {
        Self {
            table: HashMap::new(),
        }
    }

    // Insert a pair into the table. DOES NOT check if the key already exists.
    pub fn insert(&mut self, position: &Disks, min_opponent_score: i8) {
        self.table
            .insert(position.get_key(), min_opponent_score as u8);
    }

    // Returns the value associated with the key, or None if the key does not exist.
    pub fn get(&self, position: &Disks) -> Option<u8> {
        self.table.get(&position.get_key()).copied()
    }
}
