use serde::Serialize;
use sqlx::{Decode, FromRow};
use bigdecimal::BigDecimal;

#[derive(FromRow, Debug, Serialize)]
pub struct Service{
    pub id:i32,
    pub name:String,
    pub duration_minutes:i32,
    pub price:BigDecimal
}

