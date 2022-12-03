use crate::util::util::GameUpdateMessage::{self, BoardState, SimpleMessage, UndoMove};
use constants::{ConnectionProtocol, WEBSOCKET_ADDRESS};
use futures::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use gloo::net::websocket::{
    futures::WebSocket,
    Message::{self, Bytes, Text},
};
use gloo::utils::errors::JsError;
use tokio::sync::{
    mpsc::{self, UnboundedReceiver, UnboundedSender},
    oneshot::{self, Receiver as OneshotReceiver, Sender as OneshotSender},
};
use wasm_bindgen_futures::spawn_local;
use yew::Callback;

use gloo::console::{error, log};

use std::{cell::RefCell, rc::Rc};

impl From<GameUpdateMessage> for Message {
    fn from(msg: GameUpdateMessage) -> Message {
        Bytes(match msg {
            BoardState(update) => ConnectionProtocol::encode_message(update),
            SimpleMessage(msg) => vec![msg],
            UndoMove(update) => ConnectionProtocol::encode_undo_message(update),
            _ => panic!("Cannot send raw Disks variant to the server."),
        })
    }
}

/// Spawns reader and writer tasks to communicate with the server
/// On success, returns:
///     an UnboundedSender to sent messages to the writer thread, which will then write to the server
///     a mutable boolean reference to store if GameUpdateMessage should be the selected column or the entire board, as determined by the server
pub fn spawn_connection_tasks(
    callback: Callback<GameUpdateMessage>,
    lobby: String,
) -> Result<(UnboundedSender<GameUpdateMessage>, Rc<RefCell<bool>>), JsError> {
    // Task communication with server
    let websocket = WebSocket::open(WEBSOCKET_ADDRESS)?;
    let (writer, reader) = websocket.split();
    // Main app communication with tasks
    let (sender, receiver) = mpsc::unbounded_channel();
    // Channel to tell the writer task when to send the lobby information to the server
    let (connection_est_sender, connection_est_receiver) = oneshot::channel();

    let send_update_as_col_num = Rc::new(RefCell::new(false));

    spawn_reader_task(
        reader,
        callback,
        connection_est_sender,
        Rc::clone(&send_update_as_col_num),
    );
    spawn_writer_task(writer, receiver, connection_est_receiver, lobby);

    Ok((sender, send_update_as_col_num))
}

/// Task to read data sent from the server
fn spawn_reader_task(
    mut reader: SplitStream<WebSocket>,
    callback: Callback<GameUpdateMessage>,
    connection_est_sender: OneshotSender<()>,
    send_update_as_col_num: Rc<RefCell<bool>>,
) {
    spawn_local(async move {
        log!("Entered reader thread.");
        // First message indicates communication was established
        if let Some(Ok(msg)) = reader.next().await {
            if let Bytes(bytes) = msg {
                if bytes.len() != 0 && bytes[0] == ConnectionProtocol::CONNECTION_SUCCESS {
                    if bytes.len() == 1 {
                        *send_update_as_col_num.borrow_mut() = true;
                    }
                    // Tell the writer task to send to the server the lobby name
                    connection_est_sender.send(()).unwrap_or_default();
                }
            }
        }
        // Read all server messages and use a callback to update the main task with new messages
        while let Some(Ok(msg)) = reader.next().await {
            match msg {
                Bytes(bytes) => {
                    if bytes.len() == 1 {
                        callback.emit(SimpleMessage(bytes[0]));
                    } else if let Ok(update) = ConnectionProtocol::decode_message(bytes) {
                        callback.emit(BoardState(update));
                    } else {
                        error!("Received unrecognizable message from server.");
                    }
                }
                _ => {
                    error!("Expected bytes but recieved text from server.")
                }
            }
        }
        log!("Exiting reader thread.");
        callback.emit(SimpleMessage(ConnectionProtocol::CONNECTION_FAILURE));
    });
}

/// Task to write data to the server
fn spawn_writer_task(
    mut writer: SplitSink<WebSocket, Message>,
    mut receiver: UnboundedReceiver<GameUpdateMessage>,
    connection_est_receiver: OneshotReceiver<()>,
    lobby: String,
) {
    spawn_local(async move {
        log!("Entered writer thread.");
        // Wait until it is confirmed that a connection is established with the server before sending the lobby name
        if let Ok(_) = connection_est_receiver.await {
            writer.send(Text(lobby)).await.unwrap();
        } else {
            log!("Connection to server failed, exiting writer thread.");
            return;
        }
        // Forward messages sent by the main task to the server
        while let Some(msg) = receiver.recv().await {
            log!(format!("Sent {:?}", msg));
            writer.send(Message::from(msg)).await.unwrap();
        }
        // If sender from the main task is dropped, we no longer need to stay connected to the server
        // Tell the server to kill the connection so it no longer keeps writing to the reader task,
        // this will allow the reader task to end sooner
        writer
            .send(Bytes(vec![ConnectionProtocol::KILL_CONNECTION]))
            .await
            .unwrap_or_default();
        log!("Exiting writer thread.");
    });
}
