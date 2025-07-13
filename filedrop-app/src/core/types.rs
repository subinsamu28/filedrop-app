use std::sync::{Arc, Mutex};

#[derive(Clone, Default)]
pub struct TransferState {
    pub filename: String,
    pub filesize: u64,
    pub transferred: u64,
    pub complete: bool,
    pub error: Option<String>,
}

pub type SharedState = Arc<Mutex<TransferState>>;
