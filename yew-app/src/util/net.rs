use yew::{Callback, MouseEvent};

use futures::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use gloo::console::{error, log};
use gloo_net::websocket::{futures::WebSocket, Message};
use wasm_bindgen_futures::spawn_local;

use std::sync::mpsc::{self, Receiver, Sender};

pub fn spawn_connection_threads(callback: Callback<u8>) -> Sender<u8> {
    let ws = WebSocket::open("ws://127.0.0.1:8081").unwrap();
    let (writer, reader) = ws.split();
    let (sender, receiver) = mpsc::channel();

    spawn_reader_thread(reader, callback);
    spawn_writer_thread(writer, receiver);

    sender
}

fn spawn_reader_thread(mut reader: SplitStream<WebSocket>, callback: Callback<u8>) {
    spawn_local(async move {
        while let Some(Ok(msg)) = reader.next().await {
            match msg {
                Message::Bytes(bytes) => {
                    log!("Received bytes!");
                    if bytes.len() > 0 {
                        callback.emit(bytes[0]);
                    } else {
                        error!("Received 0 bytes from server.");
                    }
                }
                _ => {
                    error!("Expected bytes but recieved text from server.")
                }
            }
        }
        log!("Exiting reader thread.");
    });
}

fn spawn_writer_thread(mut writer: SplitSink<WebSocket, Message>, receiver: Receiver<u8>) {
    spawn_local(async move {
        while let Ok(msg) = receiver.recv() {
            writer.send(Message::Bytes(vec![msg])).await.unwrap();
        }
        log!("Exiting writer thread.");
    });
}
