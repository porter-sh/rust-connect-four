use crate::{
    ai::ai::AI,
    util::util::{DiskColor, Disks},
};
use constants::BOARD_WIDTH;

pub struct RandomAI;

impl AI for RandomAI {
    fn get_move(&self, board: &Disks, _: DiskColor) -> u8 {
        let open_columns = board.num_open_cols();
        if open_columns == 0 {
            return 0;
        }
        let mut idx = (rand::random::<f32>() * open_columns as f32) as usize;
        for col in 0..(BOARD_WIDTH as u8) {
            if !board.is_col_full(col) {
                if idx == 0 {
                    return col;
                }
                idx -= 1;
            }
        }
        panic!("Fix the random AI, should have returned a value");
    }
}
