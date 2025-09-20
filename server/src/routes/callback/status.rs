use std::sync::Arc;

use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde_json::json;

use crate::opcode::{DispatchEvent, Opcode};
use crate::state::AppState;

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub struct UpdateServerStatusData {
    is_online: bool,
}

pub async fn post(
    State(state): State<Arc<AppState>>,
    Json(data): Json<UpdateServerStatusData>,
) -> impl IntoResponse {
    let data = json!({
        "is_online": data.is_online,
    });

    let event = json!({
        "op": Opcode::Dispatch,
        "d": data,
        "t": DispatchEvent::ServerStatusUpdate,
    });

    _ = state
        .tx()
        .send(serde_json::to_string(&event).expect("expected hardcoded JSON to be valid"));

    StatusCode::OK
}
