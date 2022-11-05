use websocket::server::sync::Server;
use websocket::server::upgrade::WsUpgrade;
use websocket::sync::server::upgrade::Buffer;
use websocket::Message;

use std::net::TcpStream;
use std::thread;
use std::time::Duration;

const PORT: u16 = 8081;

fn handle_connection(incoming: WsUpgrade<TcpStream, Option<Buffer>>) {
    if let Ok(mut client) = incoming.accept() {
        match client.send_message(&Message::text("Poggers!!!")) {
            Ok(_) => println!("Sent message to client."),
            Err(e) => println!("{}", e),
        };
        thread::sleep(Duration::new(5, 0));
        match client.send_message(&Message::text("Poggers!!!")) {
            Ok(_) => println!("Sent message to client."),
            Err(e) => println!("{}", e),
        };
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
