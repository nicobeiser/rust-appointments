use axum::{
    routing::{get,post},
    Router,
};
use sqlx::PgPool;
use tower_http::cors::CorsLayer;

use crate::{handlers::health::health, state::AppState};
use crate::handlers::service_handler::get_services;
use crate::handlers::appointment_handler::create_appointment;

pub fn create_router(pool: PgPool) -> Router {
    let state = AppState {db:pool};
  
    Router::new()
        .route("/health", get(health))
        .route("/services", get(get_services))
        .route("/appointment", post(create_appointment))
        .layer(CorsLayer::permissive())
        .with_state(state)
}