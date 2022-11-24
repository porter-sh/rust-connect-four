//! client_handler handles clients after they have been sent to a lobby
//! 
//! new_client_handler spawns threads to read and write data over websockets to clients and to communicate with the main lobby task

use crate::Client;
use super::util::{MessageFromClient, Subtasks};

#[cfg(feature = "cppintegration")]
type Message = MessageFromClient;
#[cfg(not(feature = "cppintegration"))]
use super::util::Message::{self, BoardState, SpecialMessage};

use constants::ConnectionProtocol;

use tokio::{
    sync::{
        broadcast::{Sender as BroadcastSender, Receiver as BroadcastReceiver},
        mpsc::{UnboundedSender, UnboundedReceiver}
    },
    task::{self, JoinHandle}
};
use tokio_tungstenite::tungstenite::Message::{self as WebSocketMessage, Binary};
use futures::{SinkExt, StreamExt, stream::{SplitSink, SplitStream}};

use std::sync::{Arc, Mutex};

/// new_client_handler spawns threads to read and write data over websockets to clients and to communicate with the main lobby task
/// It also tells clients whether they are playing (and as which player) or spectating
/// 
/// Async to be run as a new task whenever a new lobby is created
pub async fn new_client_handler(
    sender: UnboundedSender<Message>,
    mut new_client_receiver: UnboundedReceiver<Client>,
    game_update_sender: BroadcastSender<MessageFromClient>,
    subtasks: Arc<Mutex<Subtasks>>
) {
    // Receive new clients sent to the lobby
    while let Some(client) = new_client_receiver.recv().await {

        let (mut writer, reader) = client.split();

        task::block_in_place(|| {
            let mut subtasks = subtasks.lock().unwrap();
            let (mut player_num, mut client_type) = (0, ConnectionProtocol::IS_SPECTATOR);

            // If there are not yet two players, make this client a player
            let subtasks_len = subtasks.tasks.len();
            if subtasks_len < 2 {
                (player_num, client_type)
                    = if subtasks_len == 0 {
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
                writer.send(Binary(vec![client_type])).await.unwrap_or_default();
                // Send the current board state to the client
                writer.send(Binary(last_board_state)).await.unwrap_or_default();
                // Write to the client on game update
                client_writer(writer, game_update_receiver, player_num).await;
            });

            // Spawn the appropriate listener and store its handle (so it can be ended when clients leave / the game ends)
            if subtasks_len < 2 {
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
async fn player_listener(mut client: SplitStream<Client>, sender: UnboundedSender<Message>, player_num: u8) {

    // Read in new messages from the client
    while let Some(Ok(msg)) = client.next().await {
        if let Binary(binary) = msg {

            // Forward the message to the main lobby task
            #[cfg(feature = "cppintegration")]
            if binary.len() == 1 {
                
                sender.send(MessageFromClient {binary, player_num}).unwrap();

            } else { println!("Player sent unrecognized message."); break; }

            #[cfg(not(feature = "cppintegration"))]
            if binary.len() == 1 {
                
                sender.send(SpecialMessage(binary[0])).unwrap();

            } else if binary.len() == ConnectionProtocol::MESSAGE_SIZE {

                sender.send(BoardState(MessageFromClient {binary, player_num})).unwrap();

            } else { println!("Player sent unrecognized message."); break; }

        } else { println!("Player sent unrecognized message."); break; }
    }

    // Tell the main lobby task to kill the lobby: the player left so the game is now over
    #[cfg(feature = "cppintegration")]
    sender.send(MessageFromClient {
        binary: vec![ConnectionProtocol::KILL_CONNECTION],
        player_num
    }).unwrap();
    #[cfg(not(feature = "cppintegration"))]
    sender.send(SpecialMessage(ConnectionProtocol::KILL_CONNECTION)).unwrap();
    println!("Ending player listener.");

}

/// spectator_listener kills the respective writer task (to save on resources) whenever a spectator leaves
/// 
/// Async to be run as a new task whenever a spectator joins the lobby
async fn spectator_listener(mut client: SplitStream<Client>, client_task: JoinHandle<()>) {
    // When a message is received, check if it the spectator is killing the connection
    while let Some(Ok(msg)) = client.next().await {
        if let Binary(msg) = msg {
            if msg.len() == 1 && msg[0] == ConnectionProtocol::KILL_CONNECTION {
                break;
            }
        }
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
    mut client: SplitSink<Client, WebSocketMessage>,
    mut receiver: BroadcastReceiver<MessageFromClient>,
    player_num: u8
) {
    // Wait for a game update
    while let Ok(msg) = receiver.recv().await {
        // If this message did not come from this client, send it to the client
        if msg.player_num != player_num {
            if let Err(_) = client.send(Binary(msg.binary)).await {
                break;
            }
        }
    }
    println!("Exiting client writer for player {}.", player_num)
}