use std::hash::Hash;

use axum::{extract::State, http::StatusCode, Json};
use bcrypt::{hash, DEFAULT_COST};
use sqlx::postgres::PgQueryResult;
use uuid::Uuid;


use crate::{
    auth::{extractor::AuthUser, jwt::create_jwt}, dto::auth_dto::{
        AuthResponse, AuthUserResponse, LoginRequest, RegisterRequest
        
    }, models::user::User, state::AppState
};
use bcrypt::verify;



pub async fn register(
    State(state) : State<AppState>,
    Json(payload) : Json<RegisterRequest>
) -> Result<(StatusCode, Json<AuthResponse>), (StatusCode, String)> {
    if (payload.name.trim().is_empty()) || (payload.email.trim().is_empty()) || (payload.password.trim().is_empty()){
                return Err((
                    StatusCode::BAD_REQUEST,
                    "Algun campo esta vacio".to_string(),
            ));
    }



    let existing_user = sqlx::query_scalar::<_,i64>(
        "SELECT COUNT(*) FROM users WHERE email = $1"
    )
    .bind(&payload.email)
    .fetch_one(&state.db)
    .await
    .map_err(internal_error)?;


    if existing_user > 0 {
        return Err((StatusCode::CONFLICT, "Ya existe un usuario con ese email".to_string()));
    }

    let password_hash = hash(&payload.password, DEFAULT_COST)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Hash error: {}", e)))?;


    let user_id = Uuid::new_v4();


    let user = sqlx::query_as::<_, User>(
        r#"
        INSERT INTO users (id, name, email, password_hash, role)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id, name, email, password_hash, role, created_at
        "#
    )
    .bind(user_id)
    .bind(&payload.name)
    .bind(&payload.email)
    .bind(&password_hash)
    .bind("client")
    .fetch_one(&state.db)
    .await
    .map_err(internal_error)?;

     let token = create_jwt(&user.id.to_string(), &user.role, &state.jwt_secret)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("JWT error: {}", e)))?;

     Ok((
        StatusCode::CREATED,
        Json(AuthResponse {
            token,
            user: AuthUserResponse {
                id: user.id.to_string(),
                name: user.name,
                email: user.email,
                role: user.role,
            },
        }),
    ))

}


pub async fn me(
    State(state) : State<AppState>,
    auth_user:AuthUser
)-> Result<Json<AuthUserResponse>, (StatusCode, String)>{
    let user:User = sqlx::query_as(
        r#"
        SELECT id,name,password_hash,role,created_at
        FROM users
        WHERE id = $1
        "#
    ).bind(auth_user.user_id.parse::<uuid::Uuid>().map_err(|_| {
        (StatusCode::BAD_REQUEST, "User id inválido".to_string())
    })?)
    .fetch_one(&state.db)
    .await
    .map_err(internal_error)?;


    Ok(Json(AuthUserResponse {
        id: user.id.to_string(),
        name: user.name,
        email: user.email,
        role: user.role,
    }))
    
}


pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, (StatusCode, String)> {
    let user = sqlx::query_as::<_, User>(
        r#"
        SELECT id, name, email, password_hash, role, created_at
        FROM users
        WHERE email = $1
        "#
    )
    .bind(&payload.email)
    .fetch_optional(&state.db)
    .await
    .map_err(internal_error)?
    .ok_or((StatusCode::UNAUTHORIZED, "Credenciales inválidas".to_string()))?;

    let valid = verify(&payload.password, &user.password_hash)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Verify error: {}", e)))?;

    if !valid {
        return Err((StatusCode::UNAUTHORIZED, "Credenciales inválidas".to_string()));
    }

    let token = create_jwt(&user.id.to_string(), &user.role, &state.jwt_secret)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("JWT error: {}", e)))?;

    Ok(Json(AuthResponse {
        token,
        user: AuthUserResponse {
            id: user.id.to_string(),
            name: user.name,
            email: user.email,
            role: user.role,
        },
    }))
}









fn internal_error(error: sqlx::Error) -> (StatusCode, String) {
    (StatusCode::INTERNAL_SERVER_ERROR, format!("Database error: {}", error))
}