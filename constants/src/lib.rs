//! constants contains relavent board constants

pub const BOARD_HEIGHT: u8 = 6; // number of rows in the board
pub const BOARD_WIDTH: u8 = 7; // number of columns in the board
pub const WEBSOCKET_ADDRESS: &str = "ws://127.0.0.1:8081";
pub const LOOKUP_TABLE_SIZE: usize = 1000; // 1000 should be slightly more than 64 MB

pub struct ConnectionProtocol;

impl ConnectionProtocol {
    pub const KILL_CONNECTION: u8 = 255;
    pub const CONNECTION_FAILED: u8 = 101;

    pub const WINNING_MOVE_ADDITION: u8 = 200;

    pub const IS_PLAYER_1: u8 = 254;
    pub const IS_PLAYER_2: u8 = 253;

    pub const COL_0: u8 = 0;
    pub const COL_1: u8 = 1;
    pub const COL_2: u8 = 2;
    pub const COL_3: u8 = 3;
    pub const COL_4: u8 = 4;
    pub const COL_5: u8 = 5;
    pub const COL_6: u8 = 6;
}
