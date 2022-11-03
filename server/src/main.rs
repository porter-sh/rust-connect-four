use core::panic;
use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::thread;

fn handle_client(mut stream: TcpStream) {
    println!("New client: {:?}", stream);
    stream.write(b"This is a message from the server.").unwrap();
}

fn main() {
    if let Ok(listener) = TcpListener::bind("127.0.0.1:8081") {
        println!("Listening on port 8081");
        for stream in listener.incoming() {
            if let Ok(stream) = stream {
                thread::spawn(move || {
                    handle_client(stream);
                });
            } else {
                println!("Error. Invalid stream: {:?}", stream.unwrap_err());
            }
        }
    } else {
        panic!("Error. Could not bind to port 8081.");
    }
}
