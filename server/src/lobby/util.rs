use tokio::task::JoinHandle;

pub struct Subtasks {
    pub tasks: Vec<JoinHandle<()>>,
    pub last_board_state: u64
}

impl Default for Subtasks {
    fn default() -> Self {
        Subtasks { tasks: Vec::new(), last_board_state: 0 }
    }
}

#[derive(Debug)]
pub enum Message {
    BoardState(u64),
    SpecialMessage(u8)
}
