use crate::{
    ai::ai::AI,
    util::util::{DiskColor, Disks},
};
use constants::BOARD_WIDTH;

pub struct RandomAI;

impl AI for RandomAI {
    fn get_move(&self, board: &Disks, _: DiskColor) -> usize {
        let mut open_columns = BOARD_WIDTH;
        for col in 0..BOARD_WIDTH {
            if board[0][col] != DiskColor::Empty {
                open_columns -= 1;
            }
        }
        if open_columns == 0 {
            return 0;
        }
        let mut idx = (rand::random::<f32>() * open_columns as f32) as usize;
        for col in 0..BOARD_WIDTH {
            if board[0][col] == DiskColor::Empty {
                if idx == 0 {
                    return col;
                }
                idx -= 1;
            }
        }
        panic!("Fix the random AI, should have returned a value");
    }
}
