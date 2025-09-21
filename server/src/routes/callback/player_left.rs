use std::sync::Arc;

use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
// use chrono::{DateTime, Utc};
use serde_json::json;

use crate::opcode::{DispatchEvent, Opcode};
use crate::state::AppState;

// This should always be in sync with the Minecraft plugin's `onPlayerDisconnect` handler.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PlayerLeft {
    uuid: String,
    username: String,
    // timestamp: DateTime<Utc>,
}

/// An endpoint for the Minecraft server to notify us when a player leaves the server.
///
/// This function should always return a `200 OK` status code.
pub async fn post(
    State(state): State<Arc<AppState>>,
    // TODO: Parse the JSON data inside the function to ensure we can always return a 200 OK.
    Json(data): Json<PlayerLeft>,
) -> impl IntoResponse {
    let data = json!({
        "uuid": data.uuid,
        "username": data.username,
        // "timestamp": data.timestamp,
    });

    let event = json!({
        "op": Opcode::Dispatch,
        "d": data,
        "t": DispatchEvent::PlayerLeft,
    });

    _ = state
        .tx()
        .send(serde_json::to_string(&event).expect("expected hardcoded JSON to be valid"));

    StatusCode::OK
}
