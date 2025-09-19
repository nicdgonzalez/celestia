//! This module is responsible for defining and implementing all of the routes.

mod api;

use std::sync::Arc;

use axum::extract::ws::{Message, WebSocket};
use axum::extract::{State, WebSocketUpgrade};
use axum::response::IntoResponse;
use axum::{Router, routing};
use futures_util::{SinkExt as _, StreamExt as _};

use crate::state::AppState;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .nest("/api", api::router())
        .route("/", routing::get(get))
}

/// This function is responsible for upgrading the client to a websocket connection.
async fn get(ws: WebSocketUpgrade, State(state): State<Arc<AppState>>) -> impl IntoResponse {
    tracing::debug!("upgrading to a websocket connection");
    ws.on_upgrade(|stream| websocket_handler(stream, state))
}

/// This function handles a single websocket connection.
async fn websocket_handler(stream: WebSocket, state: Arc<AppState>) {
    // By splitting, we can send and receive at the same time.
    let (mut sender, mut receiver) = stream.split();

    // Subscribe to listen for events from our API.
    let mut rx = state.tx().subscribe();

    // When we receive a message from the broadcaster, forward it to the client via websocket.
    let mut task_forward_event = tokio::spawn(async move {
        while let Ok(event) = rx.recv().await {
            if let Err(err) = sender.send(Message::Text(event.into())).await {
                tracing::error!("failed to send event to client via websocket: {err}");
                break;
            }
        }
    });

    // Make sure the websocket hasn't closed on us.
    let mut task_receive_message = tokio::spawn(async move {
        while let Some(message) = receiver.next().await {
            match message {
                Ok(Message::Close(_)) => {
                    tracing::debug!("client websocket connection closed");
                    break;
                }
                Err(err) => {
                    tracing::error!("client websocket returned an error: {err}");
                    break;
                }
                _ => continue,
            }
        }
    });

    // When the socket closes, stop forwarding events.
    tokio::select! {
        _ = &mut task_receive_message => task_forward_event.abort(),
        _ = &mut task_forward_event => task_receive_message.abort(),
    };
}
