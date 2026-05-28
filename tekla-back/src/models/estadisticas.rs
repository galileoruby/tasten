use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};


///Modelo de errores por tecla
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Errores {
    pub tecla: char,
    pub cantidad: u16,
}

//Estadisticas de la leccion

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Estadisticas {
    pub tecla_actual: String,
    pub posicion_actual: u16,
    pub total_errores: u16,
    pub error_por_tecla: Vec<Errores>,
    pub total_caracteres_leccion: u16,
    pub caracteres_correctos: u16,
    pub tiempo_inicio: Option<DateTime<Utc>>,
    pub tiempo_fin: Option<DateTime<Utc>>,
}
