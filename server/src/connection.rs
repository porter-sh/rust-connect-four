use constants::ConnectionProtocol;

use tokio::{
    net::TcpStream,
    task
};
use tokio_tungstenite::tungstenite::{
    error::Error,
    Message::{Binary, Text}
};
use futures::{SinkExt, StreamExt};

use std::sync::{Arc, Mutex};

use crate::{lobby::lobby, Lobbies};

pub async fn handle_connection(incoming: TcpStream, lobbies: Arc<Mutex<Lobbies>>) -> Result<(), Error> {

    let mut client = tokio_tungstenite::accept_async(incoming).await?;

    client.send(Binary(vec![ConnectionProtocol::CONNECTION_SUCCESS])).await?;

    let msg = client.next().await.unwrap_or(Err(Error::AlreadyClosed))?; 
    println!("Received msg from client.");
    if let Text(lobby) = msg {
        println!("{}", lobby);
        task::block_in_place(move || {

            let lobbies_ref = Arc::clone(&lobbies);
            let lobby_name = lobby.clone();
            if let Ok(mut lobbies) = lobbies.lock() {

                if let Some(sender) = lobbies.get(&lobby) {
                    sender.send(client).unwrap_or_default();
                    println!("Sent player to lobby.");
                } else {
                    let new_client_sender = lobby::create_lobby(Box::new(
                        move || {
                            lobbies_ref.lock().unwrap().remove(&lobby_name);
                        }
                    ));
                    lobbies.insert(lobby, new_client_sender.clone());
                    new_client_sender.send(client).unwrap_or_default();
                    println!("Created lobby.");
                }

            }

        });
    }

    println!("Connection handled.");
    Ok(())

}