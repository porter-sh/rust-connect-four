//! Contains the PerfectAIHelper struct.
//! PerfectAIHelper is used by PerfectAI to do the actual computation for
//! finding the best move.

use super::position_lookup_table::PositionLookupTable;

pub struct PerfectAIHelper {
    pub max_moves_look_ahead: u8,
    // Stores a hashmap of recent calculated board states, to avoid recalculating
    pub position_lookup_table: PositionLookupTable,
}
