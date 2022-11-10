use crate::ai::ai::Ai;

pub struct RandomAi;

impl Ai for RandomAi { fn get_move(&self) -> u8 { (rand::random::<f32>() * 7) as u8 } }
