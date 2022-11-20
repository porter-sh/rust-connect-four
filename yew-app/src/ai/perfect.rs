//! Contains the PerfectAI implementation.
//! At a high level, this AI finds the best move(s) by looking at all possible moves
//! until the end of the game (or however far we set), then picks the move that will
//! guarantee the soonest win.
use super::{ai::{AI, SurvivalAI}, position_lookup_table::PositionLookupTable, util};
use crate::util::util::{DiskColor, Disks};
use constants::*;
use gloo::console::log;

pub struct PerfectAI {
    max_moves_look_ahead: u8,
    // Stores a hashmap of recent calculated board states, to avoid recalculating
    position_lookup_table: PositionLookupTable,
}

impl PerfectAI {
    const COLUMN_ORDER: [u8; BOARD_WIDTH as usize] = [3, 2, 4, 1, 5, 0, 6];
    /// Choose which column to drop the disk in given their scores.
    /// If there are multiple columns with the same score, choose one at random.
    fn random_move_from_scores(scores: [i8; BOARD_WIDTH as usize]) -> u8 {
        // find the highest score out of all the columns
        let mut max = scores[0];
        for col in 1..BOARD_WIDTH {
            if scores[col as usize] > max {
                max = scores[col as usize];
            }
        }
        // log!(
        //     "Scores: ", scores[0], scores[1], scores[2], scores[3], scores[4], scores[5], scores[6]
        // );
        // if every column is filled
        if max == -100 {
            return 0;
        }
        // find all of the columns that have the highest score
        let mut best_cols = Vec::with_capacity(BOARD_WIDTH as usize);
        for col in 0..(BOARD_WIDTH as u8) {
            if scores[col as usize] == max {
                best_cols.push(col);
            }
        }
        // chose one of the best columns at random
        match util::random_col_from_options(&best_cols) {
            Some(col) => *col,
            None => panic!("Fix the perfect AI, should have returned a value"),
        }
    }

    /// Returns a copy of the board a disk of the given color dropped in the given column.
    fn place_disk_in_copy(board: &Disks, col: u8) -> Option<Disks> {
        let mut new_board: Disks = board.clone();
        if let Ok(_) = new_board.drop_disk(col) {
            return Some(new_board);
        }
        None
    }

    ////////////////////////////////////////////////////////////////

    pub fn new(max_moves_look_ahead: u8) -> Self {
        Self {
            max_moves_look_ahead,
            position_lookup_table: PositionLookupTable::new(LOOKUP_TABLE_SIZE),
        }
    }

    /// Get the score of some board state for a given player.
    /// Score = 43 - num_moves_until_end, or 0 for draw. If the player cannot win, score = -score.
    /// Recursive alpha-beta pruning algorithm, taking advantage of the fact
    /// that the opponent's score is the opposite of the player's score to
    /// avoid checking paths that could not be better than a previous path.
    fn get_score(
        &mut self,
        board: &Disks,
        player: DiskColor,
        num_moves_into_game: u8,
        num_moves_look_ahead: u8,
        mut min_self_score: i8,
        mut min_opponent_score: i8,
    ) -> i8 {
        // check if the current player can win on this move
        for col in 0..(BOARD_WIDTH as u8) {
            if let Some(copy) = Self::place_disk_in_copy(board, col) {
                if copy.check_last_drop_won() {
                    // log!("Num moves into game: ", num_moves_into_game);
                    // log!((BOARD_HEIGHT * BOARD_WIDTH + 1) as i8 - num_moves_into_game as i8);
                    return (BOARD_HEIGHT * BOARD_WIDTH + 1) as i8 - num_moves_into_game as i8;
                }
            }
        }

        // if we've already searched deep enough, stop
        if num_moves_look_ahead == 1 {
            return 0;
        }

        let min_possible_opponent_score =
            if let Some(min_opponent_score) = self.position_lookup_table.get(board) {
                min_opponent_score
            } else {
                (BOARD_HEIGHT * BOARD_WIDTH) as i8 - (num_moves_into_game + 1) as i8
            };

        if min_possible_opponent_score < min_opponent_score {
            min_opponent_score = min_possible_opponent_score;
            // prune; we want to minimize the opponent's score, so if we can't do any better,
            // we can stop searching this path
            if min_self_score >= min_opponent_score {
                return min_opponent_score;
            }
        }

        // calculate the score of each possible move
        for col in 0..(BOARD_WIDTH as usize) {
            if let Some(board) = Self::place_disk_in_copy(board, Self::COLUMN_ORDER[col]) {
                let score = -self.get_score(
                    &board,
                    if player == DiskColor::P1 {
                        DiskColor::P2
                    } else {
                        DiskColor::P1
                    },
                    num_moves_into_game + 1,
                    num_moves_look_ahead - 1,
                    -min_opponent_score,
                    -min_self_score,
                );

                if score >= min_opponent_score {
                    return score;
                }
                if score > min_self_score {
                    min_self_score = score;
                }
            }
        }

        self.position_lookup_table.insert(board, min_self_score);
        min_self_score
    }
}

impl AI for PerfectAI {
    /// Returns which column the AI would drop a disk into.
    fn get_move(&mut self, board: &Disks, player: DiskColor) -> u8 {
        // start each column with a bad score
        let mut score = [-100; BOARD_WIDTH as usize];
        let num_moves_into_game = board.get_num_disks();
        // calculate the actual score of each column
        for col in 0..(BOARD_WIDTH as u8) {
            if let Some(board) = Self::place_disk_in_copy(board, col) {
                // if going in one column results in a win, set the score to the best possible score.
                if board.check_last_drop_won() {
                    score[col as usize] =
                        (BOARD_HEIGHT * BOARD_WIDTH + 1) as i8 - num_moves_into_game as i8;
                } else {
                    // otherwise, calculate the score of the board state after the move
                    score[col as usize] = -self.get_score(
                        &board,
                        if player == DiskColor::P1 {
                            DiskColor::P2
                        } else {
                            DiskColor::P1
                        },
                        num_moves_into_game + 1,
                        self.max_moves_look_ahead,
                        -100,
                        100,
                    );
                }
            }
        }
        log!(self.position_lookup_table.get_num_entries());
        // Chose any one of the best columns at random (if there are multiple).
        Self::random_move_from_scores(score)
    }
}

impl SurvivalAI for PerfectAI {
    /// Used for survival mode, to make the AI harder each round.
    fn increment_difficulty(&mut self) {
        self.max_moves_look_ahead += 1;
    }
}