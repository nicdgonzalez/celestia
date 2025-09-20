use std::sync::Arc;

use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
// use chrono::{DateTime, Utc};
use serde_json::json;

use crate::opcode::{DispatchEvent, Opcode};
use crate::state::AppState;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PlayerJoined {
    uuid: String,
    username: String,
    // timestamp: DateTime<Utc>,
}

pub async fn post(
    State(state): State<Arc<AppState>>,
    Json(data): Json<PlayerJoined>,
) -> impl IntoResponse {
    let data = json!({
        "uuid": data.uuid,
        "username": data.username,
        // "timestamp": data.timestamp,
    });

    let event = json!({
        "op": Opcode::Dispatch,
        "d": data,
        "t": DispatchEvent::PlayerJoined,
    });

    _ = state
        .tx()
        .send(serde_json::to_string(&event).expect("expected hardcoded JSON to be valid"));

    StatusCode::OK
}
