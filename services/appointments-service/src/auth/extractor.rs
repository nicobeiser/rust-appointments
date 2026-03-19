use axum::{
    extract::FromRequestParts,
    http::{header::AUTHORIZATION, request::Parts, StatusCode},
};

use crate::{
    auth::jwt::decode_jwt,
    state::AppState,
};

#[derive(Debug, Clone)]
pub struct AuthUser {
    pub user_id: String,
    pub role: String,
}

impl FromRequestParts<AppState> for AuthUser {
    type Rejection = (StatusCode, String);

    fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> impl std::future::Future<Output = Result<Self, Self::Rejection>> + Send {
        let auth_header = parts
            .headers
            .get(AUTHORIZATION)
            .and_then(|value| value.to_str().ok())
            .map(str::to_owned);

        let jwt_secret = state.jwt_secret.clone();

        async move {
            let auth_header = auth_header
                .ok_or((StatusCode::UNAUTHORIZED, "Falta header Authorization".to_string()))?;

            let token = auth_header
                .strip_prefix("Bearer ")
                .ok_or((StatusCode::UNAUTHORIZED, "Formato de token inválido".to_string()))?;

            let claims = decode_jwt(token, &jwt_secret)
                .map_err(|_| (StatusCode::UNAUTHORIZED, "Token inválido o expirado".to_string()))?;

            Ok(AuthUser {
                user_id: claims.sub,
                role: claims.role,
            })
        }
    }
}