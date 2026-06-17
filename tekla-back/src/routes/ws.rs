use axum::{
    extract::{ws::{Message, WebSocket}, Path, Query, State, WebSocketUpgrade},
    response::IntoResponse,
};
use futures_util::{SinkExt, StreamExt};
use serde::Deserialize;
use tracing::{info, warn};
use crate::state::{AppState, EventoSala};

#[derive(Deserialize)]
pub struct WsParams {
    pub usuario: String,
}

#[derive(Debug, serde::Deserialize)]
#[serde(tag = "tipo", rename_all = "snake_case")]
pub enum MensajeCliente {
    Progreso {
        posicion: u16,
        errores: u16,
        caracteres_correctos: u16,
        tiempo_inicio_ms: i64,
    },
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

    // Registrar jugador en la sala
    state.unir_jugador(&room_id, &usuario);

    let (mut sink, mut stream) = socket.split();

    // Enviar texto de la carrera
    if let Some((id, texto)) = state.texto_sala(&room_id) {
        let palabras   = texto.split_whitespace().count();
        let caracteres = texto.len();
        let evento = EventoSala::TextoCarrera { id, texto, caracteres, palabras };
        let _ = sink.send(Message::Text(serde_json::to_string(&evento).unwrap().into())).await;
    }

    // Notificar entrada — incluye snapshot del estado actual de la sala
    // Un jugador tardío ve el progreso de los demás inmediatamente
    let estado_sala  = state.estado_jugadores(&room_id);
    let total        = state.total_jugadores(&room_id);
    let evento_entrada = EventoSala::JugadorUnido {
        usuario: usuario.clone(),
        total_jugadores: total,
        estado_sala,   // <-- snapshot
    };
    let _ = tx.send(serde_json::to_string(&evento_entrada).unwrap());

    // Task: broadcast → cliente
    let mut recv_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if sink.send(Message::Text(msg.into())).await.is_err() { break; }
        }
    });

    // Task: cliente → broadcast
    let usuario_clone = usuario.clone();
    let room_clone    = room_id.clone();
    let state_clone   = state.clone();
    let tx_clone      = tx.clone();

    let mut send_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = stream.next().await {
            let Message::Text(txt) = msg else {
                if matches!(msg, Message::Close(_)) { break; }
                continue;
            };

            match serde_json::from_str::<MensajeCliente>(&txt) {

                Ok(MensajeCliente::Progreso { posicion, errores, caracteres_correctos, tiempo_inicio_ms }) => {
                    let elapsed_s = {
                        let now_ms = chrono::Utc::now().timestamp_millis();
                        ((now_ms - tiempo_inicio_ms).max(1) as f64) / 1000.0
                    };
                    let wpm       = (caracteres_correctos as f64 / 5.0) / (elapsed_s / 60.0);
                    let precision = if posicion > 0 {
                        (caracteres_correctos as f64 / posicion as f64) * 100.0
                    } else { 100.0 };

                    // Guardar en memoria
                    state_clone.actualizar_progreso(
                        &room_clone, &usuario_clone,
                        posicion, errores, precision, wpm,
                    );

                    info!(
                        "[PROGRESO] sala={} usuario={} pos={} err={} wpm={:.1} prec={:.1}%",
                        room_clone, usuario_clone, posicion, errores, wpm, precision
                    );

                    let evento = EventoSala::Progreso {
                        usuario: usuario_clone.clone(),
                        posicion, errores, precision, wpm,
                    };
                    let _ = tx_clone.send(serde_json::to_string(&evento).unwrap());
                }

                Ok(MensajeCliente::Termino { tiempo_segundos, errores, caracteres_correctos, total_caracteres }) => {
                    let wpm = if tiempo_segundos > 0 {
                        (caracteres_correctos as f64 / 5.0) / (tiempo_segundos as f64 / 60.0)
                    } else { 0.0 };
                    let precision = if total_caracteres > 0 {
                        (caracteres_correctos as f64 / total_caracteres as f64) * 100.0
                    } else { 0.0 };

                    // Guardar resultado final y obtener posición en ranking
                    let ranking = state_clone.marcar_terminado(
                        &room_clone, &usuario_clone,
                        tiempo_segundos, precision, wpm,
                    );

                    info!(
                        "[TERMINO] sala={} usuario={} tiempo={}s wpm={:.1} ranking=#{}",
                        room_clone, usuario_clone, tiempo_segundos, wpm, ranking
                    );

                    let evento = EventoSala::JugadorTermino {
                        usuario: usuario_clone.clone(),
                        tiempo_segundos, precision, wpm,
                        posicion_ranking: ranking,
                    };
                    let _ = tx_clone.send(serde_json::to_string(&evento).unwrap());
                }

                Err(e) => {
                    warn!("Mensaje inválido de {}: {} — raw: {}", usuario_clone, e, txt);
                }
            }
        }
    });

    tokio::select! {
        _ = &mut recv_task => send_task.abort(),
        _ = &mut send_task => recv_task.abort(),
    }

    // Limpiar jugador y notificar salida
    state.remover_jugador(&room_id, &usuario);
    let evento_salida = EventoSala::JugadorSalio { usuario: usuario.clone() };
    let _ = tx.send(serde_json::to_string(&evento_salida).unwrap());
    info!("WS cerrado: sala={} usuario={}", room_id, usuario);
}