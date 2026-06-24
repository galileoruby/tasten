use dashmap::DashMap;
use std::sync::Arc;
use tokio::sync::broadcast;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "tipo", rename_all = "snake_case")]
pub enum EventoSala {
    Progreso {
        usuario: String,
        posicion: u16,
        errores: u16,
        precision: f64,
        wpm: f64,
    },
    JugadorTermino {
        usuario: String,
        tiempo_segundos: i64,
        precision: f64,
        wpm: f64,
        posicion_ranking: usize, // <-- nuevo: posición en el ranking
    },
    JugadorUnido {
        usuario: String,
        total_jugadores: usize,
        // Snapshot del estado actual — jugador tardío ve el progreso previo
        estado_sala: Vec<EstadoJugador>,
    },
    JugadorSalio {
        usuario: String,
    },
    TextoCarrera {
        id: usize,
        texto: String,
        caracteres: usize,
        palabras: usize,
    },
    Error {
        mensaje: String,
    },
}

/// Estado de un jugador dentro de una sala
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EstadoJugador {
    pub usuario: String,
    pub posicion: u16,
    pub errores: u16,
    pub precision: f64,
    pub wpm: f64,
    pub terminado: bool,
    pub tiempo_segundos: Option<i64>,
    pub conectado_en: DateTime<Utc>,
}

impl EstadoJugador {
    pub fn nuevo(usuario: String) -> Self {
        Self {
            usuario,
            posicion: 0,
            errores: 0,
            precision: 100.0,
            wpm: 0.0,
            terminado: false,
            tiempo_segundos: None,
            conectado_en: Utc::now(),
        }
    }
}

/// Sala activa con estado completo
pub struct Sala {
    pub tx: broadcast::Sender<String>,
    pub leccion_id: usize,
    pub texto: String,
    // Estado de cada jugador — String = nombre de usuario
    pub jugadores: DashMap<String, EstadoJugador>,
}

#[derive(Clone)]
pub struct AppState {
    pub salas: Arc<DashMap<String, Sala>>,
}

impl AppState {
    pub fn new() -> Self {
        Self { salas: Arc::new(DashMap::new()) }
    }

    pub async fn obtener_o_crear_sala(&self, room_id: &str) -> broadcast::Sender<String> {
        if let Some(sala) = self.salas.get(room_id) {
            return sala.tx.clone();
        }

        let leccion = crate::services::carrera::obtener_leccion_aleatoria().await
            .unwrap_or_else(|error| {
                tracing::warn!("No se pudo cargar la lección desde la BD: {error}");
                crate::models::carrera::LeccionResponse {
                    id: 0,
                    texto: "No se pudo cargar la lección".to_string(),
                    caracteres: 0,
                    palabras: 0,
                }
            });

        let (tx, _) = broadcast::channel(64);
        self.salas.insert(room_id.to_string(), Sala {
            tx: tx.clone(),
            leccion_id: leccion.id,
            texto: leccion.texto,
            jugadores: DashMap::new(),
        });
        tx
    }

    /// Registrar jugador al conectarse
    pub fn unir_jugador(&self, room_id: &str, usuario: &str) {
        if let Some(sala) = self.salas.get(room_id) {
            sala.jugadores.insert(
                usuario.to_string(),
                EstadoJugador::nuevo(usuario.to_string()),
            );
        }
    }

    /// Actualizar progreso de un jugador
    pub fn actualizar_progreso(
        &self, room_id: &str, usuario: &str,
        posicion: u16, errores: u16, precision: f64, wpm: f64,
    ) {
        if let Some(sala) = self.salas.get(room_id) {
            if let Some(mut jugador) = sala.jugadores.get_mut(usuario) {
                jugador.posicion  = posicion;
                jugador.errores   = errores;
                jugador.precision = precision;
                jugador.wpm       = wpm;
            }
        }
    }

    /// Marcar jugador como terminado y devolver su posición en el ranking
    pub fn marcar_terminado(
        &self, room_id: &str, usuario: &str, tiempo_segundos: i64,
        precision: f64, wpm: f64,
    ) -> usize {
        if let Some(sala) = self.salas.get(room_id) {
            if let Some(mut jugador) = sala.jugadores.get_mut(usuario) {
                jugador.terminado       = true;
                jugador.tiempo_segundos = Some(tiempo_segundos);
                jugador.precision       = precision;
                jugador.wpm             = wpm;
            }
            // Posición = cuántos ya terminaron antes que él
            return sala.jugadores.iter()
                .filter(|j| j.terminado && j.usuario != usuario)
                .count() + 1;
        }
        1
    }

    /// Eliminar jugador al desconectarse
    pub fn remover_jugador(&self, room_id: &str, usuario: &str) {
        if let Some(sala) = self.salas.get(room_id) {
            sala.jugadores.remove(usuario);
            // Limpiar sala si quedó vacía
            if sala.jugadores.is_empty() {
                drop(sala);
                self.salas.remove(room_id);
            }
        }
    }

    pub fn texto_sala(&self, room_id: &str) -> Option<(usize, String)> {
        self.salas.get(room_id).map(|s| (s.leccion_id, s.texto.clone()))
    }

    /// Snapshot del estado actual de todos los jugadores
    pub fn estado_jugadores(&self, room_id: &str) -> Vec<EstadoJugador> {
        self.salas.get(room_id)
            .map(|s| s.jugadores.iter().map(|j| j.clone()).collect())
            .unwrap_or_default()
    }

    pub fn total_jugadores(&self, room_id: &str) -> usize {
        self.salas.get(room_id)
            .map(|s| s.jugadores.len())
            .unwrap_or(0)
    }
}