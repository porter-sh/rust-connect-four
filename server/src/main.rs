use websocket::server::sync::Server;
use websocket::server::upgrade::WsUpgrade;
use websocket::sync::server::upgrade::Buffer;
use websocket::{Message, OwnedMessage};

use std::net::TcpStream;
use std::thread;
use std::time::Duration;

const PORT: u16 = 8081;

fn handle_connection(incoming: WsUpgrade<TcpStream, Option<Buffer>>) {
    if let Ok(mut client) = incoming.accept() {
        loop {
            match client.send_message(&Message::binary(&[7u8][..])) {
                Ok(_) => println!("Sent message to client."),
                Err(e) => {
                    eprintln!("{}", e);
                    break;
                }
            }
            match client.recv_message() { // onfail (when client read and write (at least read) drops) -> Ok, but do not receive bytes response
                Ok(msg) => {
                    match msg {
                        OwnedMessage::Binary(bin) => {
                            println!("Received: {}.", bin[0]);
                            if bin[0] == 255 {
                                match client.send_message(&Message::binary(bin)) {
                                    _ => ()
                                }
                                break;
                            }
                        }
                        _ => eprintln!("Did not receive bytes response.")
                    }
                }
                Err(e) => {
                    eprintln!("{}", e);
                    break;
                }
            }
        }
        match client.shutdown() {
            Ok(_) => println!("Shutdown connection to client."),
            Err(e) => println!("{}", e),
        };
    }
    println!("Exiting handle_connection thread.");
}

fn main() {
    if let Ok(mut listener) = Server::bind(format!("127.0.0.1:{}", PORT)) {
        while let Ok(incoming) = listener.accept() {
            thread::spawn(move || handle_connection(incoming));
        }
    } else {
        eprintln!("Failed to bind to port {}.", PORT);
    }
}
