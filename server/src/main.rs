use constants::ConnectionProtocol;

use tokio::{
    net::{TcpListener, TcpStream},
    sync::mpsc::{self, UnboundedSender, UnboundedReceiver},
    task
};
use tokio_tungstenite::{
    tungstenite::{
        error::Error,
        Message::{Binary, Text}
    },
    WebSocketStream
};
use futures::{SinkExt, StreamExt};

use std::{
    collections::HashMap,
    sync::{Arc, Mutex}
};

const PORT: u16 = 8081;

type Lobbies = HashMap<String, UnboundedSender<WebSocketStream<TcpStream>>>;

async fn handle_connection(incoming: TcpStream, lobbies: Arc<Mutex<Lobbies>>) -> Result<(), Error> {

    let mut client = tokio_tungstenite::accept_async(incoming).await?;

    client.send(Binary(vec![ConnectionProtocol::CONNECTION_SUCCESS])).await?;

    let msg = client.next().await.unwrap_or(Err(Error::AlreadyClosed))?; 
    println!("Received msg from client.");
    if let Text(lobby) = msg {
        println!("{}", lobby);
        task::block_in_place(move || {

            if let Ok(lobbies) = lobbies.lock() {

                if let Some(sender) = lobbies.get(&lobby) {
                    println!("Sent player to lobby.");
                    sender.send(client).unwrap_or_default();
                } else {
                    println!("Implement lobby creation.");
                }

            }

        });
    }

    println!("Exiting now");
    Ok(())

}

#[tokio::main]
async fn main() -> std::io::Result<()> {

    let listener = TcpListener::bind(format!("127.0.0.1:{}", PORT)).await?;
    let lobbies = Arc::new(Mutex::new(Lobbies::new()));

    loop {
        let (incoming, _) = listener.accept().await?;
        let lobbies = lobbies.clone();
        tokio::spawn(async move {
            if let Err(_) = handle_connection(incoming, lobbies).await {
                println!("Connection failed to properly handle.");
            }
        });
    }

}
