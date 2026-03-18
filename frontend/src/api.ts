import type {
    Service,
    CreateAppointmentRequest,
    CreateAppointmentResponse,
} from "./types";

const API_URL = "http://127.0.0.1:8080";

export async function fetchServices(): Promise<Service[]>{
    const response = await fetch(`${API_URL}/services`);
    if (!response.ok){
        throw new Error("Los servicios estan cargando mal");
    }

    return response.json();
}

export async function createAppointment(
    payload:CreateAppointmentRequest
) : Promise<CreateAppointmentResponse>{

    const response = await fetch(`${API_URL}/appointment`, {
        method:"POST",
        headers: {"Content-Type":"application/json"},
        body: JSON.stringify(payload),
    });

    if (!response.ok){
        throw new Error("Ocurrio un error reservando el appointment");
    }

    return response.json();

}