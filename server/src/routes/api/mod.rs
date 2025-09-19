mod gateway;
mod status;

use std::sync::Arc;

use axum::Router;
use axum::routing;

use crate::state::AppState;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/status", routing::post(status::post))
        .route("/gateway", routing::get(gateway::get))
}
