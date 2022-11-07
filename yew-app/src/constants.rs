//! constants contains relavent board constants

pub const BOARD_HEIGHT: usize = 6; // number of rows in the board
pub const BOARD_WIDTH: usize = 7; // number of columns in the board

pub struct ConnectionProtocol;

impl ConnectionProtocol {

    pub const KILL_CONNECTION: u8 = 255;

}