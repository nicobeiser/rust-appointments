use axum::{
    routing::get,
    Router,
};
use tower_http::cors::CorsLayer;

use crate::handlers::health::health;
use crate::handlers::service_handler::get_services;

pub fn create_router() -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/services", get(get_services))
        .layer(CorsLayer::permissive())
}