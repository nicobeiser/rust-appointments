use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use crate::auth::claims::Claims;


pub fn create_jwt(
    user_id:&str, role:&str, secret:&str
) -> Result<String, jsonwebtoken::errors::Error>{
    let expiration = Utc::now()
    .checked_add_signed(Duration::hours(1))
    .expect("invalid timestamp")
    .timestamp() as usize;

    let claims = Claims{
        sub:user_id.to_string(),
        role: role.to_string(),
        exp: expiration
    };

    encode(&Header::default(),
    &claims,
     &EncodingKey::from_secret(secret.as_bytes()))
}



pub fn decode_jwt(token: &str, secret:&str)
-> Result<Claims, jsonwebtoken::errors::Error>{
    let data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default()
    )?;


    Ok(data.claims)
}