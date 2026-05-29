use dashmap::DashMap;
use std::sync::Arc;
use tokio::sync::broadcast;
use serde::{Deserialize, Serialize};

/// Evento que se transmite a todos los jugadores de una sala
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "tipo", rename_all = "snake_case")]
pub enum EventoSala {
    /// Un jugador actualizó su progreso
    Progreso {
        usuario: String,
        posicion: u16,
        errores: u16,
        precision: f64,
        wpm: f64,
    },
    /// Un jugador terminó la carrera
    JugadorTermino {
        usuario: String,
        tiempo_segundos: i64,
        precision: f64,
        wpm: f64,
    },
    /// La sala tiene un nuevo jugador
    JugadorUnido {
        usuario: String,
        total_jugadores: usize,
    },
    /// Un jugador se desconectó
    JugadorSalio {
        usuario: String,
    },
    /// El texto de la carrera para esta sala
    TextoCarrera {
        id: usize,
        texto: String,
        caracteres: usize,
        palabras: usize,
    },
    /// Error
    Error {
        mensaje: String,
    },
}

/// Datos de una sala activa
pub struct Sala {
    /// Canal broadcast: todos los jugadores reciben todos los eventos
    pub tx: broadcast::Sender<String>,
    /// Texto fijo para esta sala (todos escriben lo mismo)
    pub leccion_id: usize,
    pub texto: String,
}

/// Estado global de la aplicación
#[derive(Clone)]
pub struct AppState {
    /// room_id -> Sala
    pub salas: Arc<DashMap<String, Sala>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            salas: Arc::new(DashMap::new()),
        }
    }

    /// Obtiene o crea una sala. Devuelve el Sender para suscribirse.
    pub fn obtener_o_crear_sala(&self, room_id: &str) -> broadcast::Sender<String> {
        if let Some(sala) = self.salas.get(room_id) {
            return sala.tx.clone();
        }

        // Sala nueva: asignar texto aleatorio
        let leccion = crate::services::carrera::obtener_leccion_aleatoria();
        let (tx, _) = broadcast::channel(64);
        self.salas.insert(
            room_id.to_string(),
            Sala {
                tx: tx.clone(),
                leccion_id: leccion.id,
                texto: leccion.texto,
            },
        );
        tx
    }

    /// Devuelve el texto asignado a una sala (None si no existe)
    pub fn texto_sala(&self, room_id: &str) -> Option<(usize, String)> {
        self.salas
            .get(room_id)
            .map(|s| (s.leccion_id, s.texto.clone()))
    }

    pub fn total_jugadores(&self, room_id: &str) -> usize {
        self.salas
            .get(room_id)
            .map(|s| s.tx.receiver_count())
            .unwrap_or(0)
    }
}