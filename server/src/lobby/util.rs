//! Util contains helper structs for lobbies

use constants::ConnectionProtocol;

use tokio::task::JoinHandle;

/// Struct to store handles to client reader tasks (so they can be killed when the lobby closes)
/// as well as the last board state (for when new players / spectators join)
pub struct Subtasks {
    pub tasks: Vec<JoinHandle<()>>,
    pub last_board_state: Vec<u8>,
}

/// Default, last_board_state is a board at the start of the match
impl Default for Subtasks {
    fn default() -> Self {
        Subtasks {
            tasks: Vec::new(),
            last_board_state: vec![0; ConnectionProtocol::MESSAGE_SIZE],
        }
    }
}

/// Message from the client, usually to be sent to other clients
#[cfg(not(feature = "cppintegration"))]
#[derive(Debug)]
pub enum Message {
    BoardState(MessageFromClient),
    SpecialMessage(u8),
}

/// Message from the client that is ConnectionProtocol::MESSAGE_SIZE bytes
#[derive(Debug, Clone)]
pub struct MessageFromClient {
    pub binary: Vec<u8>,
    pub player_num: u8,
}
