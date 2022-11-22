//! constants contains relavent board constants

pub const BOARD_HEIGHT: u8 = 6; // number of rows in the board
pub const BOARD_WIDTH: u8 = 7; // number of columns in the board
pub const WEBSOCKET_ADDRESS: &str = "ws://127.0.0.1:8081";
pub const LOOKUP_TABLE_SIZE: usize = 1000; // 1000 should be slightly more than 64 MB

pub struct ConnectionProtocol;

impl ConnectionProtocol {
    pub const KILL_CONNECTION: u8 = 255;
    pub const CONNECTION_SUCCESS: u8 = 100;
    pub const CONNECTION_FAILED: u8 = 101;

    pub const WINNING_MOVE_ADDITION: u8 = 200;

    pub const IS_PLAYER_1: u8 = 254;
    pub const IS_PLAYER_2: u8 = 253;
    pub const IS_SPECTATOR: u8 = 252;

    pub const SPECIAL_MESSAGE: u64 = 1 << (2 * BOARD_HEIGHT + 1);

    pub const COL_0: u8 = 0;
    pub const COL_1: u8 = 1;
    pub const COL_2: u8 = 2;
    pub const COL_3: u8 = 3;
    pub const COL_4: u8 = 4;
    pub const COL_5: u8 = 5;
    pub const COL_6: u8 = 6;

    pub fn assemble_message(bytes: Vec<u8>) -> Result<u64, ()> {
        if bytes.len() != 7 { return Err(()); }
        let mut msg = 0;
        for i in 0..7 {
            msg |= (bytes[i] as u64) << (i as u8 * (BOARD_HEIGHT + 1));
        }
        Ok(msg)
    }

    pub fn disassemble_message(msg: u64) -> Vec<u8> {
        const MAX_U8: u64 = 255;
        let mut bytes = Vec::with_capacity(7);
        for i in 0..7 {
            bytes.push(((msg >> (i * 8)) & MAX_U8) as u8);
        }
        bytes
    }
}
