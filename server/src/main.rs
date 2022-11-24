//! Server
//! Facilitates online multiplayer, including lobbies and spectating matches

use constants::WEBSOCKET_ADDRESS;

use tokio::{
    net::{TcpListener, TcpStream},
    sync::mpsc::UnboundedSender
};
use tokio_tungstenite::WebSocketStream;

use std::{
    collections::HashMap,
    sync::{Arc, Mutex}
};

mod connection;
mod lobby;

pub type Client = WebSocketStream<TcpStream>;
pub type Lobbies = HashMap<String, UnboundedSender<Client>>;

/// Main loop: listens for connection requests, and creates a task to handle each requests
/// Uses a multithreaded asynchronous runtime
#[tokio::main]
async fn main() -> std::io::Result<()> {

    let listener = TcpListener::bind(&WEBSOCKET_ADDRESS[5..]).await?;
    // "Global" storage of the lobbies in existence
    let lobbies = Arc::new(Mutex::new(Lobbies::new()));

    loop {
        // Wait for new connection requests
        let (incoming, _) = listener.accept().await?;
        // Handle the request
        let lobbies = Arc::clone(&lobbies);
        tokio::spawn(async move {
            if let Err(_) = connection::handle_connection(incoming, lobbies).await {
                println!("Connection failed to properly handle.");
            }
        });
    }

}
