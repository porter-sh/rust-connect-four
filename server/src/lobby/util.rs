//! Util contains helper structs for lobbies

/*
 * This file is part of Rust-Connect-Four
 * Copyright (C) 2022 Alexander Broihier <alexanderbroihier@gmail.com>
 * Copyright (C) 2022 Porter Shawver <portershawver@gmail.com>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published
 * by the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

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
