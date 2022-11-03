use std::io::prelude::*;
use std::net::TcpStream;

fn main() {
    let stream = TcpStream::connect("127.0.0.1:8081");
    if stream.is_ok() {
        println!("Connected to server");
        let mut message = String::new();
        match stream.unwrap().read_to_string(&mut message) {
            Ok(_) => println!("{}", message),
            Err(e) => println!("Error: {}", e),
        }
    } else {
        println!("Failed to connect to server.");
    }
}
