use sqlx::FromRow;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::NaiveDateTime;

#[derive(Debug,Serialize,Deserialize,FromRow)]
pub struct User{
    pub id:Uuid,
    pub name:String,
    pub email:String,
    pub password_hash:String,
    pub role: String,
    pub created_at:NaiveDateTime,
}