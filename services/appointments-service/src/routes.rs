use axum::{
    routing::{get,post},
    Router,
};
use sqlx::PgPool;
use tower_http::cors::CorsLayer;

use crate::{handlers::{auth_handler::{login, me, register}, health::health}, state::AppState};
use crate::handlers::service_handler::get_services;
use crate::handlers::appointment_handler::create_appointment;

pub fn create_router(pool: PgPool) -> Router {
    let jwt_secret = "Por ahora uso esto, para nada secreto".to_string();
    let state = AppState {db:pool, jwt_secret};
  
    Router::new()
        .route("/health", get(health))
        .route("/services", get(get_services))
        .route("/appointment", post(create_appointment))
        .route("/api/v1/auth/register", post(register))
        .route("/api/v1/auth/login", post(login))
        .route("/api/v1/auth/me", get(me))
        .layer(CorsLayer::permissive())
        .with_state(state)
}