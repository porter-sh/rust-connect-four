//! Utility functions to be used by multiple AI implementations.
use rand::seq::SliceRandom;

pub const AI_INCREMENT_MESSAGE: u8 = 8;
pub const PERFECT_SURVIVAL_DIFFICULTY_INCREMENT: u8 = 4;

/// Given a list of which columns to choose from, return one at random.
pub fn random_col_from_options(options: &Vec<u8>) -> Option<&u8> {
    options.choose(&mut rand::thread_rng())
}
