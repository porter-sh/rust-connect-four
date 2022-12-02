//! Contains the PerfectAI struct.
//! PerfectAI is an AI that finds the best possible move by looking ahead.

use crate::util::util::GameUpdateMessage;
use tokio::sync::mpsc::UnboundedSender;

pub struct PerfectAI {
    pub request_sender: UnboundedSender<GameUpdateMessage>,
}
