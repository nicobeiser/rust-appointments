use axum::Json;
use crate::models::service::Service;


pub async fn get_services() -> Json<Vec<Service>>{

    let services = vec![
        Service{
            id:1,
            name: "Corte clasico".to_string(),
            duration_minutes:30,
            price:5.0
        },
         Service{
            id:2,
            name: "Corte premium".to_string(),
            duration_minutes:40,
            price:8.0
        },
         Service{
            id:3,
            name: "Corte rapido".to_string(),
            duration_minutes:20,
            price:6.0
        },
        

    ];

    Json(services)
}