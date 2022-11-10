use constants::ConnectionProtocol;

use websocket::server::sync::Server;
use websocket::server::upgrade::WsUpgrade;
use websocket::sync::server::upgrade::Buffer;
use websocket::sync::Client;
use websocket::{Message, OwnedMessage};

// use std::collections::HashMap;
use std::net::TcpStream;
use std::thread;

use std::sync::mpsc::{self, Receiver, Sender};

const PORT: u16 = 8081;

fn handle_game_loop(mut player1: Client<TcpStream>, mut player2: Client<TcpStream>) {
    println!("In a new game.");
    loop {
        if let Ok(OwnedMessage::Binary(msg)) = player1.recv_message() {
            if msg.len() != 1 {
                println!("Bad message from player1.");
                break;
            }
            if msg[0] == ConnectionProtocol::KILL_CONNECTION {
                println!("Player1 leaving.");
                break;
            }
            player2
                .send_message(&Message::binary(msg))
                .unwrap_or_default();
        } else {
            println!("No and/or bad message from player1.");
            break;
        }
        if let Ok(OwnedMessage::Binary(msg)) = player2.recv_message() {
            if msg.len() != 1 {
                println!("Bad message from player2.");
                break;
            }
            if msg[0] == ConnectionProtocol::KILL_CONNECTION {
                println!("Player2 leaving.");
                break;
            }
            player1
                .send_message(&Message::binary(msg))
                .unwrap_or_default();
        } else {
            println!("No and/or bad message from player2.");
            break;
        }
    }
    player1.shutdown().unwrap_or_default();
    player2.shutdown().unwrap_or_default();
    println!("Game ended.");
}

fn handle_connection(
    incoming: WsUpgrade<TcpStream, Option<Buffer>>,
    receiver: Receiver<WsUpgrade<TcpStream, Option<Buffer>>>,
) {
    if let Ok(mut player1) = incoming.accept() {
        if let Err(_) = player1.send_message(&Message::binary(&[ConnectionProtocol::IS_PLAYER_1][..])) {
            println!("Early shutdown to client, exiting thread early.");
            player1.shutdown().unwrap_or_default();
            return;
        }
        if let Ok(incoming) = receiver.recv() {
            if let Ok(mut player2) = incoming.accept() {
                if let Err(_) = player2.send_message(&Message::binary(&[ConnectionProtocol::IS_PLAYER_2][..])) {
                    println!("Early shutdown to client, exiting thread early.");
                    player1.shutdown().unwrap_or_default();
                    player2.shutdown().unwrap_or_default();
                    return;
                } else {
                    handle_game_loop(player1, player2);
                }
            } else {
                println!("Early shutdown to client, exiting thread early.");
                player1.shutdown().unwrap_or_default();
                return;
            }
        } else {
            println!("Early shutdown to client, exiting thread early.");
            player1.shutdown().unwrap_or_default();
            return;
        }
    }
    println!("Exiting handle_connection thread.");
}

fn main() {
    if let Ok(mut listener) = Server::bind(format!("127.0.0.1:{}", PORT)) {
        let mut thread_sender: Option<Sender<WsUpgrade<TcpStream, Option<Buffer>>>> = None;
        while let Ok(incoming) = listener.accept() {
            match thread_sender {
                Some(ref sender) => match sender.send(incoming) {
                    Ok(_) => {
                        thread_sender = None;
                    }
                    Err(send_err) => {
                        // previous thread ended
                        let (sender, receiver) = mpsc::channel();
                        thread::spawn(move || handle_connection(send_err.0, receiver));
                        thread_sender = Some(sender);
                    }
                },
                _ => {
                    let (sender, receiver) = mpsc::channel();
                    thread::spawn(move || handle_connection(incoming, receiver));
                    thread_sender = Some(sender);
                }
            }
        }
    } else {
        eprintln!("Failed to bind to port {}.", PORT);
    }

    // if let Ok(mut listener) = Server::bind(format!("127.0.0.1:{}", PORT)) {
    //     let mut lobbies: HashMap<String, Option<Sender<WsUpgrade<TcpStream, Option<Buffer>>>> = HashMap::new();
    // } else {
    //     eprintln!("Failed to bind to port {}.", PORT);
    // }
}
