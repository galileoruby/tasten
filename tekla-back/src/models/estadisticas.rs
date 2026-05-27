use chrono::{DateTime, Utc};

///Modelo de errores por tecla
///
#[derive(Debug, Clone)]
pub struct Errores {
    pub tecla: char,
    pub cantidad: u16,
}

//Estadisticas de la leccion

#[derive(Debug, Clone)]
pub struct Estadisticas {
    pub tecla_actual: String,
    pub posicion_actual: u32,
    pub total_errores: u32,
    pub error_por_tecla: Vec<Errores>,
    pub total_caracteres_leccion: u32,
    pub caracteres_correctos: u32,
    pub tiempo_inicio: Option<DateTime<Utc>>,
    pub tiempo_fin: Option<DateTime<Utc>>,
}


impl Estadisticas{
        pub fn new(
        tecla_actual: String,
        posicion_actual: u32,
        total_caracteres_leccion: u32,
    ) -> Self {
        Estadisticas {
            tecla_actual,
            posicion_actual,
            total_errores: 0,
            error_por_tecla: Vec::new(),
            total_caracteres_leccion,
            caracteres_correctos: 0,
            tiempo_inicio: Some(Utc::now()),
            tiempo_fin: None,
        }
    }
}