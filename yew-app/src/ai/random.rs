use crate::ai::ai::AI;

pub struct RandomAi;

impl AI for RandomAi {
    fn get_move(&self) -> u8 {
        (rand::random::<f32>() * 7f32) as u8
    }
}
