//! Connections contains the handle_connection function,
//! which takes a websocket request, tells the client the connection was successful,
//! and places the client into the desired lobby

use constants::ConnectionProtocol;

use futures::{SinkExt, StreamExt};
use tokio::{net::TcpStream, task};
use tokio_tungstenite::tungstenite::{
    error::Error,
    Message::{Binary, Text},
};

use std::sync::{Arc, Mutex};

use crate::{lobby::lobby, Lobbies};

/// Takes a websocket request, tells the client the connection was successful,
/// and places the client into the desired lobby
///
/// Async to be run as a new task whenever a connection is established
pub async fn handle_connection(
    incoming: TcpStream,
    lobbies: Arc<Mutex<Lobbies>>,
) -> Result<(), Error> {
    // Accept the websocket request
    let mut client = tokio_tungstenite::accept_async(incoming).await?;

    // Confirm (besides the websocket handshake) the connection was successful
    // Length of the confirmation message indicated what type of message the client should send to the server
    #[cfg(feature = "cppintegration")]
    client
        .send(Binary(vec![ConnectionProtocol::CONNECTION_SUCCESS]))
        .await?;
    #[cfg(not(feature = "cppintegration"))]
    client
        .send(Binary(vec![ConnectionProtocol::CONNECTION_SUCCESS, 0]))
        .await?;

    // Get the lobby name from the client and place the client into the desired lobby
    let msg = client.next().await.unwrap_or(Err(Error::AlreadyClosed))?;
    println!("Received msg from client.");
    if let Text(lobby) = msg {
        println!("{}", lobby);
        task::block_in_place(move || {
            let lobby_name = lobby.clone();
            if let Ok(mut lobbies_map) = lobbies.lock() {
                // Send the player to the lobby if it already exists
                if let Some(sender) = lobbies_map.get(&lobby) {
                    sender.send(client).unwrap_or_default();
                    if lobby == "".to_string() {
                        lobbies_map.remove(&lobby);
                    }
                    println!("Sent player to lobby.");
                } else {
                    // If the lobby does not already exist
                    // Create a new lobby
                    let lobbies_ref = Arc::clone(&lobbies);
                    let new_client_sender = lobby::create_lobby(Box::new(move || {
                        lobbies_ref.lock().unwrap().remove(&lobby_name);
                    }));
                    lobbies_map.insert(lobby, new_client_sender.clone());
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
