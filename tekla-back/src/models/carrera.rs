use serde::Serialize;

/// Respuesta que se envía al cliente con el párrafo a tipear
#[derive(Debug, Serialize)]
pub struct LeccionResponse {
    pub id: usize,
    pub texto: String,
    pub caracteres: usize,
    pub palabras: usize,
}