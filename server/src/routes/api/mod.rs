mod gateway;
mod server;

use std::sync::Arc;

use axum::Router;
use axum::routing;

use crate::state::AppState;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .nest("/server", server::router())
        .route("/gateway", routing::get(gateway::get))
}
