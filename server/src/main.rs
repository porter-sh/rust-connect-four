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

#[tokio::main]
async fn main() -> std::io::Result<()> {

    let listener = TcpListener::bind(&WEBSOCKET_ADDRESS[5..]).await?;
    let lobbies = Arc::new(Mutex::new(Lobbies::new()));

    loop {
        let (incoming, _) = listener.accept().await?;
        let lobbies = Arc::clone(&lobbies);
        tokio::spawn(async move {
            if let Err(_) = connection::handle_connection(incoming, lobbies).await {
                println!("Connection failed to properly handle.");
            }
        });
    }

}
