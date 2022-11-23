use crate::Client;
use super::util::{
    Message::{self, BoardState, SpecialMessage},
    Subtasks,
    MessageFromClient
};

use constants::ConnectionProtocol;

use tokio::{
    sync::{
        broadcast::{Sender, Receiver},
        mpsc::{UnboundedSender, UnboundedReceiver}
    },
    task
};
use tokio_tungstenite::tungstenite::Message::{self as WebSocketMessage, Binary};
use futures::{SinkExt, StreamExt, stream::{SplitSink, SplitStream}};

use std::sync::{Arc, Mutex};

pub async fn new_client_handler(
    sender: UnboundedSender<Message>,
    mut new_client_receiver: UnboundedReceiver<Client>,
    game_update_sender: Sender<MessageFromClient>,
    subtasks: Arc<Mutex<Subtasks>>
) {
    while let Some(client) = new_client_receiver.recv().await {

        let (mut writer, reader) = client.split();

        task::block_in_place(|| {
            let mut subtasks = subtasks.lock().unwrap();
            let (mut player_num, mut client_type) = (0, ConnectionProtocol::IS_SPECTATOR);

            let subtasks_len = subtasks.tasks.len();
            if subtasks_len <= 2 {
                (player_num, client_type)
                    = if subtasks_len == 0 {
                        (1, ConnectionProtocol::IS_PLAYER_1)
                    } else {
                        (2, ConnectionProtocol::IS_PLAYER_2)
                    };
                let sender = sender.clone();
                subtasks.tasks.push(task::spawn(async move {
                    player_listener(reader, sender, player_num).await;
                }));
            }

            let game_update_receiver = game_update_sender.subscribe();
            let last_board_state = subtasks.last_board_state.clone();
            subtasks.tasks.push(task::spawn(async move {
                writer.send(Binary(vec![client_type])).await.unwrap_or_default();
                writer.send(Binary(last_board_state)).await.unwrap_or_default();
                client_writer(writer, game_update_receiver, player_num).await;
            }));
        });

    }
    println!("Exiting new client handler.");
}

async fn player_listener(mut client: SplitStream<Client>, sender: UnboundedSender<Message>, player_num: u8) {

    while let Some(Ok(msg)) = client.next().await {
        if let Binary(binary) = msg {

            if binary.len() == 1 {
                
                sender.send(SpecialMessage(binary[0])).unwrap();

            } else if binary.len() == ConnectionProtocol::MESSAGE_SIZE {

                sender.send(BoardState(MessageFromClient {binary, player_num})).unwrap();

            } else { println!("Player sent unrecognized message."); break; }

        } else { println!("Player sent unrecognized message."); break; }
    }

    sender.send(SpecialMessage(ConnectionProtocol::KILL_CONNECTION)).unwrap();
    println!("Ending player listener.");

}

async fn client_writer(
    mut client: SplitSink<Client, WebSocketMessage>,
    mut receiver: Receiver<MessageFromClient>,
    player_num: u8
) {
    while let Ok(msg) = receiver.recv().await {
        if msg.player_num != player_num {
            if let Err(_) = client.send(Binary(msg.binary)).await {
                break;
            }
        }
    }
}