use constants::{ConnectionProtocol, WEBSOCKET_ADDRESS};
use futures::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use gloo::utils::errors::JsError;
use gloo_net::websocket::{futures::WebSocket, Message::{self, Bytes, Text}};
use tokio::sync::{
    mpsc::{self, UnboundedReceiver, UnboundedSender},
    oneshot::{self, Receiver, Sender}
};
use wasm_bindgen_futures::spawn_local;
use yew::Callback;

use gloo::console::{error, log};

pub fn spawn_connection_threads(callback: Callback<u8>, lobby: String) -> Result<UnboundedSender<u8>, JsError> {
    let websocket = WebSocket::open(WEBSOCKET_ADDRESS)?;
    let (writer, reader) = websocket.split();
    let (sender, receiver) = mpsc::unbounded_channel();
    let (connection_est_sender, connection_est_receiver) = oneshot::channel();

    spawn_reader_thread(reader, callback, connection_est_sender);
    spawn_writer_thread(writer, receiver, connection_est_receiver, lobby);

    Ok(sender)
}

fn spawn_reader_thread(mut reader: SplitStream<WebSocket>, callback: Callback<u8>, connection_est_sender: Sender<()>) {
    spawn_local(async move {
        log!("Entered reader thread.");
        if let Some(Ok(msg)) = reader.next().await {
            if let Bytes(bytes) = msg {
                if bytes.len() == 1 && bytes[0] == ConnectionProtocol::CONNECTION_SUCCESS {
                    connection_est_sender.send(()).unwrap_or_default();
                }
            }
        }
        while let Some(Ok(msg)) = reader.next().await {
            match msg {
                Bytes(bytes) => {
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
    connection_est_receiver: Receiver<()>,
    lobby: String
) {
    spawn_local(async move {
        log!("Entered writer thread.");
        if let Ok(_) = connection_est_receiver.await {
            writer.send(Text(lobby)).await.unwrap();
        } else {
            log!("Connection to server failed, exiting writer thread.");
            return;
        }
        while let Some(msg) = receiver.recv().await {
            log!(format!("Sent {}", msg));
            writer.send(Bytes(vec![msg])).await.unwrap();
        }
        writer
            .send(Bytes(vec![ConnectionProtocol::KILL_CONNECTION]))
            .await
            .unwrap_or_default();
        log!("Exiting writer thread.");
    });
}
