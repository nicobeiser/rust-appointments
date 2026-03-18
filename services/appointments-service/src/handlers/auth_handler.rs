use axum::{extract::State, http::StatusCode, Json};
use bcrypt::{hash, DEFAULT_COST};
use sqlx::postgres::PgQueryResult;
use uuid::Uuid;


use crate::{
    dto::auth_dto::{
        AuthResponse,
        AuthUserResponse,
        RegisterRequest
    },
    models::user::User,
    state::AppState,
    auth::jwt::create_jwt,
};

