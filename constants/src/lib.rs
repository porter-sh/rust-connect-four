//! constants contains relavent board constants

pub const BOARD_HEIGHT: u8 = 6; // number of rows in the board
pub const BOARD_WIDTH: u8 = 7; // number of columns in the board
pub const WEBSOCKET_ADDRESS: &str = "ws://127.0.0.1:8081";
pub const LOOKUP_TABLE_SIZE: usize = 1000; // 1000 should be slightly more than 64 MB

/// Helper enum like struct to provide some communication standards between the client and server
pub struct ConnectionProtocol;

/// Helper struct to represent a game update to be sent between the client and server
#[derive(Debug, Clone)]
pub struct GameUpdate {
    pub position: u64,
    pub mask: u64,
    pub is_p1_turn: bool,
    pub game_won: bool
}

impl ConnectionProtocol {
    pub const KILL_CONNECTION: u8 = 255;
    pub const CONNECTION_SUCCESS: u8 = 100;
    pub const CONNECTION_FAILED: u8 = 101;

    pub const WINNING_MOVE_ADDITION: u8 = 200;

    pub const IS_PLAYER_1: u8 = 254;
    pub const IS_PLAYER_2: u8 = 253;
    pub const IS_SPECTATOR: u8 = 252;

    pub const COL_0: u8 = 0;
    pub const COL_1: u8 = 1;
    pub const COL_2: u8 = 2;
    pub const COL_3: u8 = 3;
    pub const COL_4: u8 = 4;
    pub const COL_5: u8 = 5;
    pub const COL_6: u8 = 6;

    /// Number of bytes in a message representing a GameUpdate to be sent over a websocket
    pub const MESSAGE_SIZE: usize = 14;
    
    const IS_NOT_P1_TURN: u64 = 1 << (2 * BOARD_HEIGHT + 1);
    const GAME_WON: u64 = 1 << (3 * BOARD_HEIGHT + 2);

    /// Turns a vector of bytes, sent over a websocket, into an easily usable GameUpdate object
    /// Fails if bytes.len() != ConnectionProtocol::MESSAGE_SIZE
    pub fn assemble_message(bytes: Vec<u8>) -> Result<GameUpdate, ()> {
        if bytes.len() != Self::MESSAGE_SIZE { return Err(()); }

        let mut position = 0;
        for i in 0..(Self::MESSAGE_SIZE / 2) {
            position |= (bytes[i] as u64) << (i * 8);
        }

        let mut mask = 0;
        for i in 0..(Self::MESSAGE_SIZE / 2) {
            mask |= (bytes[i + Self::MESSAGE_SIZE / 2] as u64) << (i * 8);
        }

        let is_p1_turn = mask & Self::IS_NOT_P1_TURN == 0;
        if !is_p1_turn {
            mask &= !Self::IS_NOT_P1_TURN;
        }

        let game_won = mask & Self::GAME_WON != 0;
        if game_won {
            mask &= !Self::GAME_WON;
        }

        Ok(GameUpdate {position, mask, is_p1_turn, game_won})
    }

    /// Turns a GameUpdate into a vector of bytes, which can be sent over a websocket
    /// The returned Vec has a length of ConnectionProtocol::MESSAGE_SIZE
    pub fn disassemble_message(mut msg: GameUpdate) -> Vec<u8> {
        const MAX_U8: u64 = std::u8::MAX as u64;

        let mut bytes = Vec::with_capacity(Self::MESSAGE_SIZE);
        for i in 0..(Self::MESSAGE_SIZE / 2) {
            bytes.push(((msg.position >> (i * 8)) & MAX_U8) as u8);
        }

        if !msg.is_p1_turn {
            msg.mask |= Self::IS_NOT_P1_TURN;
        }

        if msg.game_won {
            msg.mask |= Self::GAME_WON;
        }

        for i in 0..(Self::MESSAGE_SIZE / 2) {
            bytes.push(((msg.mask >> (i * 8)) & MAX_U8) as u8);
        }

        bytes
    }
}
