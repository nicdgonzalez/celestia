mod player_joined;
mod player_left;
mod status;

use std::sync::Arc;

use axum::Router;
use axum::routing;

use crate::state::AppState;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/status", routing::post(status::post))
        .route("/player_joined", routing::post(player_joined::post))
        .route("/player_left", routing::post(player_left::post))
}
