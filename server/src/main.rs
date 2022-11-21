use constants::ConnectionProtocol;

use tokio::{
    net::{TcpListener, TcpStream},
    sync::mpsc::{self, UnboundedSender, UnboundedReceiver}
};
use tokio_tungstenite::{
    tungstenite::Message::Binary,
    WebSocketStream
};
use futures::{SinkExt, StreamExt};

const PORT: u16 = 8081;

async fn handle_game_loop(mut player1: WebSocketStream<TcpStream>, mut player2: WebSocketStream<TcpStream>) {
    println!("In a new game.");
    loop {
        if let Some(Ok(Binary(msg))) = player1.next().await {
            if msg.len() != 1 {
                println!("Bad message from player1.");
                break;
            }
            if msg[0] == ConnectionProtocol::KILL_CONNECTION {
                println!("Player1 leaving.");
                break;
            }
            player2
                .send(Binary(msg))
                .await
                .unwrap_or_default();
        } else {
            println!("No and/or bad message from player1.");
            break;
        }
        if let Some(Ok(Binary(msg))) = player2.next().await {
            if msg.len() != 1 {
                println!("Bad message from player2.");
                break;
            }
            if msg[0] == ConnectionProtocol::KILL_CONNECTION {
                println!("Player2 leaving.");
                break;
            }
            player1
                .send(Binary(msg))
                .await
                .unwrap_or_default();
        } else {
            println!("No and/or bad message from player2.");
            break;
        }
    }
    player1.close(None).await.unwrap_or_default();
    player2.close(None).await.unwrap_or_default();
    println!("Game ended.");
}

async fn handle_connection(
    incoming: TcpStream,
    mut receiver: UnboundedReceiver<TcpStream>,
) {
    println!("Handling connection.");
    if let Ok(mut player1) = tokio_tungstenite::accept_async(incoming).await {
        if let Err(_) = player1.send(Binary(vec![ConnectionProtocol::IS_PLAYER_1])).await {
            println!("Early shutdown to client, exiting thread early.");
            player1.close(None).await.unwrap_or_default();
            return;
        }
        if let Some(incoming) = receiver.recv().await {
            if let Ok(mut player2) = tokio_tungstenite::accept_async(incoming).await {
                if let Err(_) = player2.send(Binary(vec![ConnectionProtocol::IS_PLAYER_2])).await {
                    println!("Early shutdown to client, exiting thread early.");
                    player1.close(None).await.unwrap_or_default();
                    player2.close(None).await.unwrap_or_default();
                    return;
                } else {
                    handle_game_loop(player1, player2).await;
                }
            } else {
                println!("Early shutdown to client, exiting thread early.");
                player1.close(None).await.unwrap_or_default();
                return;
            }
        } else {
            println!("Early shutdown to client, exiting thread early.");
            player1.close(None).await.unwrap_or_default();
            return;
        }
    }
    println!("Exiting handle_connection thread.");
}

#[tokio::main]
async fn main() -> std::io::Result<()> {

    let listener = TcpListener::bind(format!("127.0.0.1:{}", PORT)).await?;
    let mut thread_sender: Option<UnboundedSender<TcpStream>> = None;

    loop {
        let (incoming, _) = listener.accept().await?;
        match &thread_sender {
            Some(sender) => match sender.send(incoming) {
                Ok(_) => thread_sender = None,
                Err(send_err) => {
                    // previous thread ended
                    let (sender, receiver) = mpsc::unbounded_channel();
                    tokio::spawn(async move { handle_connection(send_err.0, receiver).await; });
                    thread_sender = Some(sender);
                }
            }
            _ => {
                let (sender, receiver) = mpsc::unbounded_channel();
                tokio::spawn(async move { handle_connection(incoming, receiver).await; });
                thread_sender = Some(sender);
            }
        }
    }

    // if let Ok(mut listener) = Server::bind(format!("127.0.0.1:{}", PORT)) {
    //     let mut lobbies: HashMap<String, Option<Sender<WsUpgrade<TcpStream, Option<Buffer>>>> = HashMap::new();
    // } else {
    //     eprintln!("Failed to bind to port {}.", PORT);
    // }
}
