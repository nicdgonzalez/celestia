mod start;
mod status;
mod stop;

use std::sync::Arc;

use axum::Router;
use axum::routing;

use crate::state::AppState;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/status", routing::get(status::get))
        .route("/start", routing::post(start::post))
        .route("/stop", routing::post(stop::post))
}
