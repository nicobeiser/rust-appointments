use axum::Json;

use crate::dto::appointment_dto::
{CreateAppointmentRequest,
CreateAppointmentResponse};


pub async fn create_appointment(
    Json(payload) : Json<CreateAppointmentRequest>
) -> Json<CreateAppointmentResponse>{
    println!("Nuevo turno recibido:");
    println!("Cliente: {}", payload.client_name);
    println!("Teléfono: {}", payload.client_phone);
    println!("Servicio ID: {}", payload.service_id);
    println!("Fecha: {}", payload.appointment_date);
    println!("Hora: {}", payload.start_time);
    println!("Notas: {:?}", payload.notes);
    
    Json(CreateAppointmentResponse { message: "Turno creado".to_string() })
} 