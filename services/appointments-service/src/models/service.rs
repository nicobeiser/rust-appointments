use serde::{Deserialize, Serialize};
use sqlx::{Decode, FromRow};
use bigdecimal::BigDecimal;

#[derive(FromRow, Debug, Serialize, Deserialize)]
pub struct Service{
    pub id:i32,
    pub name:String,
    pub duration_minutes:i32,
    pub price:BigDecimal
}

