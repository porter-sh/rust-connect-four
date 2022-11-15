use rand::seq::SliceRandom;

/// Given a list of which columns to choose from, return one at random.
pub fn random_col_from_options(options: &Vec<u8>) -> Option<&u8> {
    options.choose(&mut rand::thread_rng())
}
