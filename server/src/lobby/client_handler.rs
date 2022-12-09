//! client_handler handles clients after they have been sent to a lobby
//!
//! new_client_handler spawns threads to read and write data over websockets to clients and to communicate with the main lobby task

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

use super::util::{MessageFromClient, Subtasks};
use crate::Client;

#[cfg(feature = "cppintegration")]
type Message = MessageFromClient;
#[cfg(not(feature = "cppintegration"))]
use super::util::Message::{self, BoardState, SpecialMessage};

use constants::ConnectionProtocol;

use tokio::{
    io::{split, AsyncReadExt, AsyncWriteExt, ReadHalf, WriteHalf},
    net::TcpStream,
    sync::{
        broadcast::{Receiver as BroadcastReceiver, Sender as BroadcastSender},
        mpsc::{UnboundedReceiver, UnboundedSender},
    },
    task::{self, JoinHandle},
};
use tokio_rustls::server::TlsStream;

use std::sync::{Arc, Mutex};

/// new_client_handler spawns tasks to read and write data over websockets to clients and to communicate with the main lobby task
/// It also tells clients whether they are playing (and as which player) or spectating
///
/// Async to be run as a new task whenever a new lobby is created
pub async fn new_client_handler(
    sender: UnboundedSender<Message>,
    mut new_client_receiver: UnboundedReceiver<Client>,
    game_update_sender: BroadcastSender<MessageFromClient>,
    subtasks: Arc<Mutex<Subtasks>>,
) {
    // Receive new clients sent to the lobby
    while let Some(stream) = new_client_receiver.recv().await {
        let (reader, mut writer) = split(stream);

        task::block_in_place(|| {
            let mut subtasks = subtasks.lock().unwrap();
            let (mut player_num, mut client_type) = (0, ConnectionProtocol::IS_SPECTATOR);

            // If there are not yet two players, make this client a player
            let subtasks_len = subtasks.tasks.len();
            if subtasks_len < 2 {
                (player_num, client_type) = if subtasks_len == 0 {
                    (1, ConnectionProtocol::IS_PLAYER_1)
                } else {
                    (2, ConnectionProtocol::IS_PLAYER_2)
                };
            }

            // Spawn a task to write to the client
            // This task ends when lobby drops game_update_receiver or when the reader task receives ConnectionProtocol::KILL_CONNECTION
            let game_update_receiver = game_update_sender.subscribe();
            let last_board_state = subtasks.last_board_state.clone();
            let client_task = task::spawn(async move {
                // Send to the client which player it is, or if it is a spectator
                writer.write(&[client_type]).await.unwrap_or_default();
                if subtasks_len != 0 {
                    // Send the current board state to the client
                    writer.write(&last_board_state).await.unwrap_or_default();
                }
                // Write to the client on game update
                client_writer(writer, game_update_receiver, player_num).await;
            });

            // Spawn the appropriate listener and store its handle (so it can be ended when clients leave / the game ends)
            if subtasks_len < 2 {
                if subtasks_len == 1 {
                    #[cfg(feature = "cppintegration")]
                    sender
                        .send(Message {
                            binary: vec![ConnectionProtocol::SECOND_PLAYER_CONNECTED],
                            player_num: 2,
                        })
                        .unwrap_or_default();
                    #[cfg(not(feature = "cppintegration"))]
                    sender
                        .send(BoardState(MessageFromClient {
                            binary: vec![ConnectionProtocol::SECOND_PLAYER_CONNECTED],
                            player_num: 2,
                        }))
                        .unwrap_or_default();
                }
                let sender = sender.clone();
                subtasks.tasks.push(task::spawn(async move {
                    player_listener(reader, sender, player_num).await;
                }));
            } else {
                subtasks.tasks.push(task::spawn(async move {
                    spectator_listener(reader, client_task).await;
                }));
            }
        });
    }
    println!("Exiting new client handler.");
}

/// player_listener forwards messages received from the player to the main lobby task
/// When the player leaves, it sends ConnectionProtocol::KILL_CONNECTION as the game is now over
///
/// Async to be run as a new task whenever a player joins the lobby
async fn player_listener(
    mut client_readhalf: ReadHalf<TlsStream<TcpStream>>,
    sender: UnboundedSender<Message>,
    player_num: u8,
) {
    // Read in new messages from the client
    let mut msg_buf = [0u8; ConnectionProtocol::MESSAGE_SIZE];
    while let Ok(read_size) = client_readhalf.read(&mut msg_buf).await {
        // Forward the message to the main lobby task
        #[cfg(feature = "cppintegration")]
        if binary.len() == 1 && binary[0] != ConnectionProtocol::SECOND_PLAYER_CONNECTED {
            sender
                .send(MessageFromClient { binary, player_num })
                .unwrap_or_default();
        } else {
            println!("Player sent unrecognized message.");
            break;
        }

        #[cfg(not(feature = "cppintegration"))]
        if read_size == 1 {
            sender.send(SpecialMessage(msg_buf[0])).unwrap_or_default();
        } else if read_size == ConnectionProtocol::MESSAGE_SIZE {
            sender
                .send(BoardState(MessageFromClient {
                    binary: msg_buf.to_vec(),
                    player_num,
                }))
                .unwrap_or_default();
        } else {
            println!("Player sent unrecognized message.");
            break;
        }

        msg_buf = [0u8; ConnectionProtocol::MESSAGE_SIZE];
    }

    // Tell the main lobby task to kill the lobby: the player left so the game is now over
    #[cfg(feature = "cppintegration")]
    sender
        .send(MessageFromClient {
            binary: vec![ConnectionProtocol::KILL_CONNECTION],
            player_num,
        })
        .unwrap_or_default();
    #[cfg(not(feature = "cppintegration"))]
    sender
        .send(SpecialMessage(ConnectionProtocol::KILL_CONNECTION))
        .unwrap_or_default();
    println!("Ending player listener.");
}

/// spectator_listener kills the respective writer task (to save on resources) whenever a spectator leaves
///
/// Async to be run as a new task whenever a spectator joins the lobby
async fn spectator_listener(
    mut client_readhalf: ReadHalf<TlsStream<TcpStream>>,
    client_task: JoinHandle<()>,
) {
    // When a message is received, check if it the spectator is killing the connection
    let mut msg_buf = [0u8; 2];
    while let Ok(read_size) = client_readhalf.read(&mut msg_buf).await {
        if read_size == 1 && msg_buf[0] == ConnectionProtocol::KILL_CONNECTION {
            break;
        }
        msg_buf = [0u8; 2];
    }

    // Kill the corresponding writer task
    client_task.abort();
    println!("Killed spectator task.");
}

/// client_writer sends game updates to the client
///
/// Async to be run as a new task whenever a spectator joins the lobby
/// One task per client due to awaiting the send over a websocket
async fn client_writer(
    mut client_writehalf: WriteHalf<TlsStream<TcpStream>>,
    mut receiver: BroadcastReceiver<MessageFromClient>,
    player_num: u8,
) {
    // Wait for a game update
    while let Ok(msg) = receiver.recv().await {
        // If this message did not come from this client, send it to the client
        if msg.player_num != player_num {
            if let Err(_) = client_writehalf.write(&msg.binary).await {
                break;
            }
        }
    }
    println!("Exiting client writer for player {}.", player_num)
}
