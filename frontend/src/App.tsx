import { useEffect, useState } from "react";
import "./App.css";
import { createAppointment, fetchServices } from "./api";
import type { Service, CreateAppointmentRequest } from "./types";

function App() {
  const [services, setServices] = useState<Service[]>([]);
  const [loadingServices, setLoadingServices] = useState(true);
  const [errorServices, setErrorServices] = useState("");
  const [successMessage, setSuccessMessage] = useState("");
  const [errorSubmit, setErrorSubmit] = useState("");

  const [form, setForm] = useState<CreateAppointmentRequest>({
    client_name: "",
    client_phone: "",
    service_id: 0,
    appointment_date: "",
    start_time: "",
    notes: "",
  });

  useEffect(() => {
    async function loadServices() {
      try {
        const data = await fetchServices();
        setServices(data);

        if (data.length > 0) {
          setForm((prev) => ({
            ...prev,
            service_id: data[0].id,
          }));
        }
      } catch (error) {
        setErrorServices("Error cargando los servicios");
      } finally {
        setLoadingServices(false);
      }
    }

    loadServices();
  }, []);

  function handleChange(
    event: React.ChangeEvent<HTMLInputElement | HTMLSelectElement | HTMLTextAreaElement>
  ) {
    const { name, value } = event.target;

    setForm((prev) => ({
      ...prev,
      [name]: name === "service_id" ? Number(value) : value,
    }));
  }

  async function handleSubmit(event: React.FormEvent<HTMLFormElement>) {
    event.preventDefault();
    setSuccessMessage("");
    setErrorSubmit("");

    try {
      const payload: CreateAppointmentRequest = {
        ...form,
        notes: form.notes?.trim() ? form.notes : undefined,
      };

      const response = await createAppointment(payload);
      setSuccessMessage(response.message);

      setForm((prev) => ({
        client_name: "",
        client_phone: "",
        service_id: services.length > 0 ? services[0].id : 0,
        appointment_date: "",
        start_time: "",
        notes: "",
      }));
    } catch (error) {
      setErrorSubmit("No se pudo enviar el turno");
    }
  }

  return (
    <div className="container">
      <h1>Reservar turno</h1>

      <section className="card">
        <h2>Servicios disponibles</h2>

        {loadingServices && <p>Cargando servicios...</p>}
        {errorServices && <p className="error">{errorServices}</p>}

        {!loadingServices && !errorServices && (
          <ul className="services-list">
            {services.map((service) => (
              <li key={service.id} className="service-item">
                <strong>{service.name}</strong>
                <span>{service.duration_minutes} min</span>
                <span>${service.price}</span>
              </li>
            ))}
          </ul>
        )}
      </section>

      <section className="card">
        <h2>Crear turno</h2>

        <form onSubmit={handleSubmit} className="form">
          <input
            type="text"
            name="client_name"
            placeholder="Nombre del cliente"
            value={form.client_name}
            onChange={handleChange}
            required
          />

          <input
            type="text"
            name="client_phone"
            placeholder="Teléfono"
            value={form.client_phone}
            onChange={handleChange}
            required
          />

          <select
            name="service_id"
            value={form.service_id}
            onChange={handleChange}
            required
          >
            {services.map((service) => (
              <option key={service.id} value={service.id}>
                {service.name}
              </option>
            ))}
          </select>

          <input
            type="date"
            name="appointment_date"
            value={form.appointment_date}
            onChange={handleChange}
            required
          />

          <input
            type="time"
            name="start_time"
            value={form.start_time}
            onChange={handleChange}
            required
          />

          <textarea
            name="notes"
            placeholder="Notas"
            value={form.notes}
            onChange={handleChange}
          />

          <button type="submit">Reservar</button>
        </form>

        {successMessage && <p className="success">{successMessage}</p>}
        {errorSubmit && <p className="error">{errorSubmit}</p>}
      </section>
    </div>
  );
}

export default App;