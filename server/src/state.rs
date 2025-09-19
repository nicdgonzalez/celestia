use tokio::sync::broadcast::Sender;

#[derive(Debug, Clone)]
pub struct AppState {
    tx: Sender<String>,
}

impl AppState {
    pub fn new(tx: Sender<String>) -> Self {
        Self { tx }
    }

    /// A reference to the channel for sending events to all connected clients.
    pub const fn tx(&self) -> &Sender<String> {
        &self.tx
    }
}
