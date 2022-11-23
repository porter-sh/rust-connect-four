use constants::ConnectionProtocol;

use tokio::task::JoinHandle;

pub struct Subtasks {
    pub tasks: Vec<JoinHandle<()>>,
    pub last_board_state: Vec<u8>
}

impl Default for Subtasks {
    fn default() -> Self {
        Subtasks { tasks: Vec::new(), last_board_state: vec![0; ConnectionProtocol::MESSAGE_SIZE] }
    }
}

#[derive(Debug)]
pub enum Message {
    BoardState(MessageFromClient),
    SpecialMessage(u8)
}

#[derive(Debug, Clone)]
pub struct MessageFromClient {
    pub binary: Vec<u8>,
    pub player_num: u8
}
