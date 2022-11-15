use crate::{
    ai::ai::AI,
    util::util::{DiskColor, DiskData, Disks},
};
use constants::*;

use std::cmp::min;

pub struct PerfectAI {
    max_moves_look_ahead: u8,
}

impl PerfectAI {
    fn random_move_from_scores(scores: [i8; 7]) -> usize {
        let mut max = scores[0];
        for col in 1..BOARD_WIDTH {
            if scores[col] > max {
                max = scores[col];
            }
        }
        // if every column is filled
        if max == -100 {
            return 0;
        }
        let mut best_cols = Vec::with_capacity(BOARD_WIDTH);
        for col in 0..BOARD_WIDTH {
            if scores[col] == max {
                best_cols.push(col);
            }
        }

        let idx = (rand::random::<f32>() * best_cols.len() as f32) as usize;
        best_cols[idx]
    }

    fn place_disk_in_copy(board: &Disks, col: usize, player: DiskColor) -> Option<Disks> {
        for row in (0..BOARD_HEIGHT).rev() {
            if board.position[row][col] == DiskColor::Empty {
                let board = *board;
                board.position[row][col] = player;
                return Some(board);
            }
        }
        None
    }

    fn is_winning_move(board: &Disks, col: usize, player: DiskColor) -> Option<bool> {
        for row in (0..BOARD_HEIGHT).rev() {
            if board.position[row][col] == DiskColor::Empty {
                let new_disk = DiskData::new(row, col, player);
                board.check_winner(new_disk);
                return Some(board.can_move);
            }
        }
        return None;
    }

    fn get_moves_in_game(board: &Disks) -> u8 {
        let num_moves = 0;
        for col in 0..BOARD_WIDTH {
            for row in (0..BOARD_HEIGHT).rev() {
                if board.position[row][col] == DiskColor::Empty {
                    break;
                }
                num_moves += 1;
            }
        }
        num_moves
    }

    //////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    //////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    //////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    //////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    //////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    //////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    //////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    //////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    //////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    //////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    //////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    //////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    //////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    //////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

    pub fn new(max_moves_look_ahead: u8) -> Self {
        Self {
            max_moves_look_ahead,
        }
    }

    fn get_score(
        board: &Disks,
        player: DiskColor,
        num_moves_into_game: u8,
        num_moves_look_ahead: u8,
        min_self_score: i8,
        min_opponent_score: i8,
    ) -> i8 {
        if num_moves_into_game as usize == BOARD_HEIGHT * BOARD_WIDTH {
            return 0;
        }

        for col in 0..BOARD_WIDTH {
            if let Some(_) = Self::place_disk_in_copy(board, col, player) {
                if Self::is_winning_move(board, col, player).unwrap() {
                    return (BOARD_HEIGHT * BOARD_WIDTH) as i8 - num_moves_into_game as i8;
                }
            }
        }

        if num_moves_look_ahead == 1 {
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

        for col in 0..BOARD_WIDTH {
            if let Some(board) = Self::place_disk_in_copy(board, col, player) {
                let score = Self::get_score(
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
    fn get_move(&self, board: &Disks, player: DiskColor) -> usize {
        let mut score = [-100; BOARD_WIDTH];
        for col in 0..BOARD_WIDTH {
            if let Some(board) = Self::place_disk_in_copy(board, col, player) {
                score[col] = Self::get_score(
                    &board,
                    player,
                    Self::get_moves_in_game(&board),
                    self.max_moves_look_ahead,
                    -100,
                    100,
                );
            }
        }

        Self::random_move_from_scores(score)
    }
}
