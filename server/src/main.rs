//! Server
//! Facilitates online multiplayer, including lobbies and spectating matches

/*
 * This file is part of Rust-Connect-Four
 * Copyright (C) 2022 Alexander Broihier <alexanderbroihier@gmail.com>
 * Copyright (C) 2022 Porter Shawver <portershawver@gmail.com>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published
 * by the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

use tokio::{
    net::{TcpListener, TcpStream},
    sync::mpsc::UnboundedSender,
};
use tokio_tungstenite::WebSocketStream;

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

mod connection;
mod lobby;

#[cfg(feature = "cppintegration")]
mod bindings;

type Client = WebSocketStream<TcpStream>;
type Lobbies = HashMap<String, UnboundedSender<Client>>;

/// Main loop: listens for connection requests, and creates a task to handle each requests
/// Uses a multithreaded asynchronous runtime
#[tokio::main]
async fn main() -> std::io::Result<()> {
    #[cfg(feature = "cppintegration")]
    println!("C++ integration enabled.");
    #[cfg(not(feature = "cppintegration"))]
    println!("C++ integration disabled.");

    let listener = TcpListener::bind("172.31.36.17:8080").await?;
    // "Global" storage of the lobbies in existence
    let lobbies = Arc::new(Mutex::new(Lobbies::new()));

    loop {
        // Wait for new connection requests
        let (incoming, _) = listener.accept().await?;
        // Handle the request
        let lobbies = Arc::clone(&lobbies);
        tokio::spawn(async move {
            if let Err(_) = connection::handle_connection(incoming, lobbies).await {
                println!("Client failed to connect.");
            }
        });
    }
}
