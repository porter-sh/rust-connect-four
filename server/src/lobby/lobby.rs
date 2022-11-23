use crate::Client;
use super::{
    client_handler,
    util::{
        Message::{self, BoardState, SpecialMessage},
        Subtasks, MessageFromClient
    }
};

use tokio::{
    sync::{
        broadcast::{self, Sender},
        mpsc::{self, UnboundedSender, UnboundedReceiver}
    },
    task
};

use std::sync::{Arc, Mutex};

async fn run_lobby(
    mut receiver: UnboundedReceiver<Message>,
    game_update_sender: Sender<MessageFromClient>,
    subtasks: Arc<Mutex<Subtasks>>,
    remove_lobby: Box<dyn FnOnce() -> () + Send + Sync>
) {

    while let Some(msg) = receiver.recv().await {
        match msg {

            BoardState(state) => {
                task::block_in_place(|| {
                    subtasks.lock().unwrap().last_board_state = state.binary.clone();
                });
                game_update_sender.send(state).unwrap_or_default();
            }
            SpecialMessage(_) => {
                break;
            }

        }
    }

    task::block_in_place(move || {
        remove_lobby();

        for subtask in &subtasks.lock().unwrap().tasks {
            subtask.abort();
        }
    });
    println!("Ending lobby.");
}

pub fn create_lobby(remove_lobby: Box<dyn FnOnce() -> () + Send + Sync>) -> UnboundedSender<Client> {

    let (sender, receiver) = mpsc::unbounded_channel();
    let (new_client_sender, new_client_receiver)
        = mpsc::unbounded_channel();

    let (game_update_sender, _) = broadcast::channel(3);
    let game_update_sender_clone = game_update_sender.clone();

    let subtasks = Arc::new(Mutex::new(Subtasks::default()));
    let subtasks_ref = Arc::clone(&subtasks);

    task::spawn(async move {
        run_lobby(receiver, game_update_sender, subtasks, remove_lobby).await;
    });
    task::spawn(async move {
        client_handler::new_client_handler(sender, new_client_receiver, game_update_sender_clone, subtasks_ref).await;
    });
    
    new_client_sender

}