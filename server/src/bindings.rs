use constants::{ConnectionProtocol, GameUpdate};

use std::os::raw::c_int;

#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq)]
enum DiskType {
    kPlayer1 = 82,
    kPlayer2 = 66,
    kEmpty = 32,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Board {
    board: [[DiskType; Board::kBoardWidth as usize]; Board::kBoardHeight as usize],
}

impl Default for Board {
    fn default() -> Self {
        Self {
            board: [[DiskType::kEmpty; Board::kBoardWidth as usize]; Board::kBoardHeight as usize],
        }
    }
}

impl Board {
    #[allow(non_upper_case_globals)]
    pub const kBoardWidth: c_int = 7;
    #[allow(non_upper_case_globals)]
    pub const kBoardHeight: c_int = 6;

    pub fn make_move(&mut self, player_num: u8, col: u8) -> Result<bool, ()> {
        let disk = if player_num == 1 {
            DiskType::kPlayer1
        } else {
            DiskType::kPlayer2
        };
        unsafe {
            if DropDiskToBoardSucceeded(self as *mut Board, disk, col as c_int) {
                Ok(CheckForWinner(self as *mut Board, disk))
            } else {
                Err(())
            }
        }
    }
    pub fn to_game_update_binary(&self, is_p1_turn: bool, game_won: bool) -> Vec<u8> {
        let (mut position, mut mask) = (0, 0);
        let mut bit: u64 = 1;
        for col in 0..Board::kBoardWidth {
            for row in 0..Board::kBoardHeight {
                let disk = self.board[row as usize][col as usize];
                if disk != DiskType::kEmpty {
                    mask |= bit;
                    if is_p1_turn == (disk == DiskType::kPlayer1) {
                        position |= bit;
                    }
                }
                bit <<= 1;
            }
            bit <<= 1;
        }
        ConnectionProtocol::encode_message(GameUpdate {
            position,
            mask,
            is_p1_turn,
            game_won,
        })
    }
}

extern "C" {
    fn DropDiskToBoardSucceeded(b: *mut Board, disk: DiskType, col: c_int) -> bool;
    fn CheckForWinner(b: *mut Board, disk: DiskType) -> bool;
}
