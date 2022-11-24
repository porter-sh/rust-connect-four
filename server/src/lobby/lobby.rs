//! lobby contains create_lobby, which allows players to create and join lobbies

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
        broadcast::{self, Sender as BroadcastSender},
        mpsc::{self, UnboundedSender, UnboundedReceiver}
    },
    task
};

use std::sync::{Arc, Mutex};

/// run_lobby is the main task for each lobby and accordingly handles the lifecycle of the lobby
/// 
/// Async to be run as a new task whenever a lobby is created
async fn run_lobby(
    mut receiver: UnboundedReceiver<Message>,
    game_update_sender: BroadcastSender<MessageFromClient>,
    subtasks: Arc<Mutex<Subtasks>>,
    remove_lobby: Box<dyn FnOnce() -> () + Send + Sync>
) {

    // Is player1's turn at the start of the game
    let mut is_p1_turn = true;
    // When player input is received
    while let Some(msg) = receiver.recv().await {
        match msg {

            // If a message was received from the player whose turn it was, store the updated game state and send it to all clients
            BoardState(state) => {
                if is_p1_turn == (state.player_num == 1) {
                    task::block_in_place(|| {
                        subtasks.lock().unwrap().last_board_state = state.binary.clone();
                    });
                    game_update_sender.send(state).unwrap_or_default();
                    is_p1_turn = !is_p1_turn;
                }
            }
            // Special messages at this stage means a player killed the connection
            SpecialMessage(_) => {
                break;
            }

        }
    }

    // Delete this lobby and kill all tasks listening to players
    // All writer tasks will end once the senders to them are dropped
    task::block_in_place(move || {
        remove_lobby();

        for subtask in &subtasks.lock().unwrap().tasks {
            subtask.abort();
        }
    });
    println!("Ending lobby.");
}

/// create_lobby starts the run_lobby and new_client_handler tasks for the given lobby
/// Returns a sender which can send new clients to the lobby
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