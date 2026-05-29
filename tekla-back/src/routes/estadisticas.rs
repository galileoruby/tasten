use axum::{routing::post, Json, Router};
use serde_json::{json, Value};

use crate::{
    models::carrera::LeccionResponse, 
    models::estadisticas::Estadisticas,
    services::carrera::obtener_leccion_aleatoria, 
    services::estadisticas::EstadisticasService,
};

pub async fn registrar_resultados(Json(estadistica): Json<Estadisticas>) -> Json<Value> {
    Json(EstadisticasService::registrar_resultados(&estadistica))
}

pub fn router_estadisticas() -> Router {
    Router::new().route("/api/estadisticas/registrar", post(registrar_resultados))
}
