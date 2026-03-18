export type Service = {
    id: number;
    name: string;
    duration_minutes: number;
    price: number;
};

export type CreateAppointmentRequest = {
    client_name: string;
    client_phone: string;
    service_id: number;
    appointment_date: string;
    start_time: string;
    notes?: string;
};

export type CreateAppointmentResponse = {
    message: string;
};