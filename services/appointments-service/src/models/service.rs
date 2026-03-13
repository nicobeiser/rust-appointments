use serde::Serialize;

#[derive(Serialize)]
pub struct Service{
    pub id:u32,
    pub name:String,
    pub duration_minutes:u32,
    pub price:f32
}

