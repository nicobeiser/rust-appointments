use serde::{Serialize, Deserialize};


#[derive(Deserialize)]
pub struct CreateAppointmentRequest{
    pub client_name:String,
    pub client_phone:String,
    pub service_id:u32,
    pub appointment_date: String,
    pub start_time: String,
    pub notes: Option<String>
}

#[derive(Serialize)]
pub struct CreateAppointmentResponse{
    pub message:String,
}