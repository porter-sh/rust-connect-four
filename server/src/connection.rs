//! Connections contains the handle_connection function,
//! which takes a websocket request, tells the client the connection was successful,
//! and places the client into the desired lobby

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

/// Takes a websocket request, tells the client the connection was successful,
/// and places the client into the desired lobby
/// 
/// Async to be run as a new task whenever a connection is established
pub async fn handle_connection(incoming: TcpStream, lobbies: Arc<Mutex<Lobbies>>) -> Result<(), Error> {

    // Accept the websocket request
    let mut client = tokio_tungstenite::accept_async(incoming).await?;

    // Confirm (besides the websocket handshake) the connection was successful
    // Length of the confirmation message indicated what type of message the client should send to the server
    client.send(Binary(vec![ConnectionProtocol::CONNECTION_SUCCESS, 0])).await?;

    // Get the lobby name from the client and place the client into the desired lobby
    let msg = client.next().await.unwrap_or(Err(Error::AlreadyClosed))?; 
    println!("Received msg from client.");
    if let Text(lobby) = msg {
        println!("{}", lobby);
        task::block_in_place(move || {

            let lobbies_ref = Arc::clone(&lobbies);
            let lobby_name = lobby.clone();
            if let Ok(mut lobbies) = lobbies.lock() {

                // Send the player to the lobby if it already exists
                if let Some(sender) = lobbies.get(&lobby) {
                    sender.send(client).unwrap_or_default();
                    println!("Sent player to lobby.");
                } else { // If the lobby does not already exist
                    // Create a new lobby
                    let new_client_sender = lobby::create_lobby(Box::new(
                        move || {
                            lobbies_ref.lock().unwrap().remove(&lobby_name);
                        }
                    ));
                    lobbies.insert(lobby, new_client_sender.clone());
                    // Send the player to the new lobby
                    new_client_sender.send(client).unwrap_or_default();
                    println!("Created lobby.");
                }

            }

        });
    }

    println!("Connection handled.");
    Ok(())

}