use crate::models::estadisticas::{Errores, Estadisticas};
use serde_json::{json, Value};
use chrono::Utc;

/// Servicio para manejar la lógica de negocio de estadísticas
pub struct EstadisticasService;

impl EstadisticasService {
    /*
    /// Crear una nueva instancia de estadísticas    
    pub fn crear(tecla_actual: String, total_caracteres_leccion: u16, id_leccion: u16) -> Estadisticas {
        Estadisticas {
            id_leccion,
            tecla_actual,
            posicion_actual: 0,
            total_errores: 0,
            error_por_tecla: Vec::new(),
            total_caracteres_leccion,
            caracteres_correctos: 0,
            tiempo_inicio: Some(Utc::now()),
            tiempo_fin: None,
        }
    }
    */

    pub fn calcular_precision(estadisticas: &Estadisticas) -> f64 {
        if estadisticas.total_caracteres_leccion == 0 {
            return 0.0;
        }
        (estadisticas.caracteres_correctos as f64 / estadisticas.total_caracteres_leccion as f64)
            * 100.0
    }

        /// Calcular tiempo transcurrido en segundos
    pub fn calcular_tiempo_transcurrido(estadisticas: &Estadisticas) -> Option<i64> {
        match (&estadisticas.tiempo_inicio, &estadisticas.tiempo_fin) {
            (Some(inicio), Some(fin)) => {
                let duracion = fin.signed_duration_since(*inicio);
                Some(duracion.num_seconds())
            }
            _ => None,
        }
    }

    /// Calcular velocidad (caracteres por minuto)
    pub fn calcular_velocidad(estadisticas: &Estadisticas) -> f64 {
        let tiempo_segundos = match (&estadisticas.tiempo_inicio, &estadisticas.tiempo_fin) {
            (Some(inicio), Some(fin)) => {
                let duracion = fin.signed_duration_since(*inicio);
                duracion.num_seconds() as f64
            }
            _ => return 0.0,
        };

        if tiempo_segundos == 0.0 {
            return 0.0;
        }

        // Caracteres por minuto = (caracteres correctos / segundos) * 60
        (estadisticas.caracteres_correctos as f64 / tiempo_segundos) * 60.0
    }

    /// Registrar resultados finales de una lección
    /// Recibe el objeto Estadísticas y retorna un JSON con estado, mensaje y datos
    pub fn registrar_resultados(estadisticas: &Estadisticas) -> Value {



        // Validar que la lección esté finalizada
        if estadisticas.id_leccion.is_none() {
            return json!({
                "estado": "error",
                "mensaje": "Identificacion de leccion ausente",
                "exitoso": false,
                "datos": null
            });
        }
        if estadisticas.tiempo_fin.is_none() {
            return json!({
                "estado": "error",
                "mensaje": "La lección no ha sido finalizada",
                "exitoso": false,
                "datos": null
            });
        }

        // Calcular métricas
        let precision = Self::calcular_precision(estadisticas);
        let velocidad = Self::calcular_velocidad(estadisticas);
        let tiempo_transcurrido = Self::calcular_tiempo_transcurrido(estadisticas);

        // Validar que se haya completado la lección
        let lecccion_completada =
            estadisticas.caracteres_correctos == estadisticas.total_caracteres_leccion;

        // Determinar estado basado en precisión
        let estado_precision = match precision {
            p if p >= 95.0 => "excelente",
            p if p >= 85.0 => "muy_bueno",
            p if p >= 75.0 => "bueno",
            p if p >= 60.0 => "aceptable",
            _ => "necesita_mejora",
        };

       return   json!({
            "estado": "exitoso",
            "mensaje": "Resultados registrados correctamente",
            "exitoso": true,
            "datos": {
                "leccion": {
                    "tecla_actual": estadisticas.tecla_actual,
                    "total_caracteres": estadisticas.total_caracteres_leccion,
                    "completada": lecccion_completada,
                },
                "metricas": {
                    "caracteres_correctos": estadisticas.caracteres_correctos,
                    "total_errores": estadisticas.total_errores,
                    "precision": format!("{:.2}%", precision),
                    "precision_valor": format!("{:.2}", precision),
                    "velocidad_cpm": format!("{:.2}", velocidad),
                    "estado_precision": estado_precision,
                    "tiempo_segundos": tiempo_transcurrido,
                },
                "errores": {
                    "total": estadisticas.total_errores,
                    "por_tecla": estadisticas.error_por_tecla.clone(),
                },
                "tiempos": {
                    "inicio": estadisticas.tiempo_inicio.map(|t| t.to_rfc3339()),
                    "fin": estadisticas.tiempo_fin.map(|t| t.to_rfc3339()),
                    "duracion_segundos": tiempo_transcurrido,
                },
            }
        })
    }
}
