use axum::{
    routing::get,
    Router,
};
use tower_http::cors::CorsLayer;

use crate::handlers::health::health;

pub fn create_router() -> Router {
    Router::new()
        .route("/health", get(health))
        .layer(CorsLayer::permissive())
}