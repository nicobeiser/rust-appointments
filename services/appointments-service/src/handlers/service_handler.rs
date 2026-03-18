use axum::{extract::State, Json};
use sqlx::Postgres;
use crate::models::service::Service;
use crate::state::AppState;


pub async fn get_services(
    State(state): State<AppState>
) -> Result<Json<Vec<Service>>,(axum::http::StatusCode, String)>{

let pool: &sqlx::PgPool = &state.db;

let services:Vec<Service> = sqlx::query_as(
    "SELECT id, name, duration_minutes, price FROM services ORDER BY id"
)
.fetch_all(pool)
.await
.map_err(internal_error)?;

    Ok(Json(services))
}

fn internal_error(error: sqlx::Error) -> (axum::http::StatusCode, String) {
    (
        axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        format!("Database error: {}", error),
    )
}   


async fn get_service_names(
    State(state) :  State<AppState>
) -> Result<Json<Vec<String>>,(axum::http::StatusCode, String)>{

    let pool = &state.db;
    let names:Result<Vec<String>, (axum::http::StatusCode, String)> = sqlx::query_scalar(
        "SELECT name FROM services"
    )
    .fetch_all(pool)
    .await
    .map_err(internal_error);
    let result = names?;

    Ok(Json(result))

}