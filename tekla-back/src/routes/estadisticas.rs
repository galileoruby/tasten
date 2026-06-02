use axum::{routing::post, Json, Router};
use serde_json::{ Value};

use crate::{
    
    models::estadisticas::Estadisticas,
    
    services::estadisticas::EstadisticasService,
};

pub async fn registrar_resultados(Json(estadistica): Json<Estadisticas>) -> Json<Value> {
    Json(EstadisticasService::registrar_resultados(&estadistica))
}

pub fn router_estadisticas() -> Router {
    Router::new().route("/api/estadisticas/registrar", post(registrar_resultados))
}
