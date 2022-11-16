use super::{ai::AI, util};
use crate::util::util::{DiskColor, Disks};
use constants::*;
use gloo::console::log;

pub struct PerfectAI {
    max_moves_look_ahead: u8,
}

impl PerfectAI {
    /// Choose which column to drop the disk in given their scores.
    /// If there are multiple columns with the same score, choose one at random.
    fn random_move_from_scores(scores: [i8; BOARD_WIDTH as usize]) -> u8 {
        // find max score
        let mut max = scores[0];
        for col in 1..BOARD_WIDTH {
            if scores[col as usize] > max {
                max = scores[col as usize];
            }
        }
        log!(
            "Scores: ", scores[0], scores[1], scores[2], scores[3], scores[4], scores[5], scores[6]
        );
        // if every column is filled
        if max == -100 {
            return 0;
        }
        // find all of the best columns
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
        }
    }

    /// Get the score of some board state for a given player.
    /// Score = 43 - num_moves_until_end, or 0 for draw. If the player cannot win, score = -score.
    /// Recursive alpha-beta pruning algorithm, taking advantage of the fact
    /// that the opponent's score is the opposite of the player's score to
    /// avoid checking paths that could not be better than a previous path.
    fn get_score(
        board: &Disks,
        player: DiskColor,
        num_moves_into_game: u8,
        num_moves_look_ahead: u8,
        mut min_self_score: i8,
        mut min_opponent_score: i8,
    ) -> i8 {
        /*if num_moves_into_game == BOARD_HEIGHT * BOARD_WIDTH {
            return 0;
        }*/

        for col in 0..(BOARD_WIDTH as u8) {
            if let Some(copy) = Self::place_disk_in_copy(board, col) {
                if copy.check_last_drop_won() {
                    // log!("Num moves into game: ", num_moves_into_game);
                    // log!((BOARD_HEIGHT * BOARD_WIDTH + 1) as i8 - num_moves_into_game as i8);
                    return (BOARD_HEIGHT * BOARD_WIDTH + 1) as i8 - num_moves_into_game as i8;
                }
            }
        }

        if num_moves_look_ahead == 1 {
            log!("Looked ahead far enough.");
            return 0;
        }

        let min_possible_opponent_score =
            (BOARD_HEIGHT * BOARD_WIDTH) as i8 - 1 - num_moves_into_game as i8;
        if min_possible_opponent_score < min_opponent_score {
            min_opponent_score = min_possible_opponent_score;
            if min_self_score >= min_opponent_score {
                return min_opponent_score;
            }
        }

        for col in 0..(BOARD_WIDTH as u8) {
            if let Some(board) = Self::place_disk_in_copy(board, col) {
                let score = -Self::get_score(
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

        min_self_score
    }
}

impl AI for PerfectAI {
    fn get_move(&self, board: &Disks, player: DiskColor) -> u8 {
        let mut score = [-100; BOARD_WIDTH as usize];
        let num_moves_into_game = board.get_num_disks();
        for col in 0..(BOARD_WIDTH as u8) {
            if let Some(board) = Self::place_disk_in_copy(board, col) {
                if board.check_last_drop_won() {
                    score[col as usize] =
                        (BOARD_HEIGHT * BOARD_WIDTH + 1) as i8 - num_moves_into_game as i8;
                } else {
                    score[col as usize] = -Self::get_score(
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

        Self::random_move_from_scores(score)
    }
}
