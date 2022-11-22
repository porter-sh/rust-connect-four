use constants::ConnectionProtocol;

use tokio::{
    net::TcpStream,
    sync::{
        broadcast::{self, Sender, Receiver},
        mpsc::{self, UnboundedSender, UnboundedReceiver}
    },
    task::{self, JoinHandle}
};
use tokio_tungstenite::{
    tungstenite::Message::Binary,
    WebSocketStream
};
use futures::{SinkExt, StreamExt, stream::{SplitSink, SplitStream}};

use std::{
    collections::HashMap,
    sync::{Arc, Mutex}
};

pub type Client = WebSocketStream<TcpStream>;
pub type Lobbies = HashMap<String, UnboundedSender<Client>>;

struct Subtasks {
    tasks: Vec<JoinHandle<()>>,
    last_board_state: u64
}

impl Default for Subtasks {
    fn default() -> Self {
        Subtasks { tasks: Vec::new(), last_board_state: 0 }
    }
}

#[derive(Debug)]
enum Message {
    BoardState(u64),
    SpecialMessage(u8)
}

use Message::{BoardState, SpecialMessage};

async fn run_lobby(
    mut receiver: UnboundedReceiver<Message>,
    game_update_sender: Sender<Vec<u8>>,
    subtasks: Arc<Mutex<Subtasks>>,
    remove_lobby: Box<dyn FnOnce() -> () + Send + Sync>
) {

    while let Some(msg) = receiver.recv().await {
        match msg {

            BoardState(state) => {
                { subtasks.lock().unwrap().last_board_state = state; }
                game_update_sender.send(ConnectionProtocol::disassemble_message(state)).unwrap_or_default();
            }
            SpecialMessage(_) => {
                break;
            }

        }
    }

    task::block_in_place(move || {
        remove_lobby();

        for subtask in &subtasks.lock().unwrap().tasks {
            subtask.abort();
        }
    });
    println!("Ending lobby.");
}

async fn new_client_handler(
    sender: UnboundedSender<Message>,
    mut new_client_receiver: UnboundedReceiver<Client>,
    game_update_sender: Sender<Vec<u8>>,
    subtasks: Arc<Mutex<Subtasks>>
) {
    while let Some(client) = new_client_receiver.recv().await {

        let (mut writer, reader) = client.split();

        task::block_in_place(|| {
            let mut subtasks = subtasks.lock().unwrap();
            let mut client_type = ConnectionProtocol::IS_SPECTATOR;

            let subtasks_len = subtasks.tasks.len();
            if subtasks_len <= 2 { // TODO !!! UPDATE length number
                client_type = if subtasks_len == 0 { ConnectionProtocol::IS_PLAYER_1 } else { ConnectionProtocol::IS_PLAYER_2 };
                let sender = sender.clone();
                subtasks.tasks.push(task::spawn(async move {
                    player_listener(reader, sender).await;
                }));
            }

            let game_update_receiver = game_update_sender.subscribe();
            let last_board_state = subtasks.last_board_state;
            subtasks.tasks.push(task::spawn(async move {
                writer.send(Binary(vec![client_type])).await.unwrap_or_default();
                writer.send(Binary(ConnectionProtocol::disassemble_message(last_board_state))).await.unwrap_or_default();
                client_writer(writer, game_update_receiver).await;
            }));
        });

    }
    println!("Exiting new client handler.");
}

async fn player_listener(mut client: SplitStream<Client>, sender: UnboundedSender<Message>) {

    while let Some(Ok(msg)) = client.next().await {
        if let Binary(binary) = msg {

            if binary.len() == 1 {
                
                sender.send(SpecialMessage(binary[0])).unwrap();

            } else {

                if let Ok(msg) = ConnectionProtocol::assemble_message(binary) {
                    sender.send(BoardState(msg)).unwrap();
                } else { println!("Player sent unrecognized message."); break; }

            }

        } else { println!("Player sent unrecognized message."); break; }
    }

    sender.send(SpecialMessage(ConnectionProtocol::KILL_CONNECTION)).unwrap();
    println!("Ending player listener.");

}

async fn client_writer(mut client: SplitSink<Client, tokio_tungstenite::tungstenite::Message>, mut receiver: Receiver<Vec<u8>>) {
    while let Ok(binary) = receiver.recv().await {
        if let Err(_) = client.send(Binary(binary)).await {
            break;
        }
    }
}

pub fn create_lobby(remove_lobby: Box<dyn FnOnce() -> () + Send + Sync>) -> UnboundedSender<Client> {

    let (sender, receiver) = mpsc::unbounded_channel();
    let (new_client_sender, new_client_receiver)
        = mpsc::unbounded_channel();

    let (game_update_sender, _) = broadcast::channel(3);
    let game_update_sender_clone = game_update_sender.clone();

    let subtasks = Arc::new(Mutex::new(Subtasks::default()));
    let subtasks_ref = Arc::clone(&subtasks);

    task::spawn(async move {
        run_lobby(receiver, game_update_sender, subtasks, remove_lobby).await;
    });
    task::spawn(async move {
        new_client_handler(sender, new_client_receiver, game_update_sender_clone, subtasks_ref).await;
    });
    
    new_client_sender

}