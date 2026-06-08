use axum::{
    extract::{
        ws::{Message, WebSocket},
        Path, Query, State, WebSocketUpgrade,
    },
    response::IntoResponse,
};
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use tracing::{info, warn};

use crate::state::{AppState, EventoSala};

/// Query params: /ws/:room_id?usuario=juan
#[derive(Deserialize)]
pub struct WsParams {
    pub usuario: String,
}

/// Mensajes que el cliente puede enviar al servidor
#[derive(Debug, Deserialize)]
#[serde(tag = "tipo", rename_all = "snake_case")]
pub enum MensajeCliente {
    /// El cliente actualiza su posición y errores mientras escribe
    Progreso {
        posicion: u16,
        errores: u16,
        caracteres_correctos: u16,
        tiempo_inicio_ms: i64, // timestamp epoch ms desde que empezó
    },
    /// El cliente terminó la carrera
    Termino {
        tiempo_segundos: i64,
        errores: u16,
        caracteres_correctos: u16,
        total_caracteres: u16,
    },
}

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    Path(room_id): Path<String>,
    Query(params): Query<WsParams>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let usuario = params.usuario.clone();
    info!("WS upgrade: sala={} usuario={}", room_id, usuario);
    ws.on_upgrade(move |socket| manejar_socket(socket, room_id, usuario, state))
}

async fn manejar_socket(socket: WebSocket, room_id: String, usuario: String, state: AppState) {
    let tx = state.obtener_o_crear_sala(&room_id);
    let mut rx = tx.subscribe();

    let (mut sink, mut stream) = socket.split();

    // --- Enviar el texto de la carrera al recién conectado ---
    if let Some((id, texto)) = state.texto_sala(&room_id) {
        let palabras = texto.split_whitespace().count();
        let caracteres = texto.len();
        let evento = EventoSala::TextoCarrera {
            id,
            texto,
            caracteres,
            palabras,
        };
        let json = serde_json::to_string(&evento).unwrap();
        let _ = sink.send(Message::Text(json.into())).await;
    }

    // --- Notificar a la sala que llegó un nuevo jugador ---
    let total = state.total_jugadores(&room_id);
    let evento_entrada = EventoSala::JugadorUnido {
        usuario: usuario.clone(),
        total_jugadores: total,
    };
    let _ = tx.send(serde_json::to_string(&evento_entrada).unwrap());

    // --- Task: reenviar broadcast -> este cliente ---
    let mut recv_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if sink.send(Message::Text(msg.into())).await.is_err() {
                break;
            }
        }
    });

    // --- Loop: leer mensajes del cliente ---
    let usuario_clone = usuario.clone();
    let room_clone = room_id.clone();
    let tx_clone = tx.clone();

    let mut send_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = stream.next().await {
            match msg {
                Message::Text(txt) => {
                    match serde_json::from_str::<MensajeCliente>(&txt) {
                        Ok(MensajeCliente::Progreso {
                            posicion,
                            errores,
                            caracteres_correctos,
                            tiempo_inicio_ms,
                        }) => {
                            // Calcular WPM y precisión en el servidor
                            let elapsed_s = {
                                let now_ms = chrono::Utc::now().timestamp_millis();
                                ((now_ms - tiempo_inicio_ms).max(1) as f64) / 1000.0
                            };
                            let wpm = (caracteres_correctos as f64 / 5.0) / (elapsed_s / 60.0);
                            let precision = if posicion > 0 {
                                (caracteres_correctos as f64 / posicion as f64) * 100.0
                            } else {
                                100.0
                            };

                            info!(
                            "[PROGRESO] usuario={}, pos={}, err={}, correctos={}, elapsed_s={:.2}, wpm={:.2}, precision={:.2}",
                            usuario_clone, posicion, errores, caracteres_correctos, elapsed_s, wpm, precision
                            );

                            let evento = EventoSala::Progreso {
                                usuario: usuario_clone.clone(),
                                posicion,
                                errores,
                                precision,
                                wpm,
                            };
                            let _ = tx_clone.send(serde_json::to_string(&evento).unwrap());
                        }
                        Ok(MensajeCliente::Termino {
                            tiempo_segundos,
                            errores,
                            caracteres_correctos,
                            total_caracteres,
                        }) => {
                            let wpm = if tiempo_segundos > 0 {
                                (caracteres_correctos as f64 / 5.0)
                                    / (tiempo_segundos as f64 / 60.0)
                            } else {
                                0.0
                            };
                            let precision = if total_caracteres > 0 {
                                (caracteres_correctos as f64 / total_caracteres as f64) * 100.0
                            } else {
                                0.0
                            };

                            let evento = EventoSala::JugadorTermino {
                                usuario: usuario_clone.clone(),
                                tiempo_segundos,
                                precision,
                                wpm,
                            };
                            let _ = tx_clone.send(serde_json::to_string(&evento).unwrap());
                        }
                        Err(e) => {
                            warn!(
                                "Mensaje inválido de {}: {} — raw: {}",
                                usuario_clone, e, txt
                            );
                        }
                    }
                }
                Message::Close(_) => break,
                _ => {}
            }
        }
    });

    // Cuando cualquiera de las dos tasks termina, cancelar la otra
    tokio::select! {
        _ = &mut recv_task => send_task.abort(),
        _ = &mut send_task => recv_task.abort(),
    }

    // Notificar salida
    let evento_salida = EventoSala::JugadorSalio {
        usuario: usuario.clone(),
    };
    let _ = tx.send(serde_json::to_string(&evento_salida).unwrap());
    info!("WS cerrado: sala={} usuario={}", room_id, usuario);
}
