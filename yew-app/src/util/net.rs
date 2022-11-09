use crate::constants::{ConnectionProtocol, WEBSOCKET_ADDRESS};
use futures::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use gloo::utils::errors::JsError;
use gloo_net::websocket::{futures::WebSocket, Message};
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};
use wasm_bindgen_futures::spawn_local;
use yew::Callback;

use gloo::console::{error, log};

pub fn spawn_connection_threads(callback: Callback<u8>) -> Result<UnboundedSender<u8>, JsError> {
    let websocket = WebSocket::open(WEBSOCKET_ADDRESS)?;
    let (writer, reader) = websocket.split();
    let (sender, receiver) = mpsc::unbounded_channel();

    spawn_reader_thread(reader, callback);
    spawn_writer_thread(writer, receiver);

    Ok(sender)
}

fn spawn_reader_thread(mut reader: SplitStream<WebSocket>, callback: Callback<u8>) {
    spawn_local(async move {
        log!("Entered reader thread.");
        while let Some(Ok(msg)) = reader.next().await {
            match msg {
                Message::Bytes(bytes) => {
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

fn spawn_writer_thread(
    mut writer: SplitSink<WebSocket, Message>,
    mut receiver: UnboundedReceiver<u8>,
) {
    spawn_local(async move {
        log!("Entered writer thread.");
        while let Some(msg) = receiver.recv().await {
            log!(format!("Sent {}", msg));
            writer.send(Message::Bytes(vec![msg])).await.unwrap();
        }
        writer
            .send(Message::Bytes(vec![ConnectionProtocol::KILL_CONNECTION]))
            .await
            .unwrap_or_default();
        log!("Exiting writer thread.");
    });
}
