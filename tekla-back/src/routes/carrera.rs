use axum::{http::StatusCode, routing::get, Json, Router};

use crate::{
    models::carrera::LeccionResponse,
    services::carrera::obtener_leccion_aleatoria,
};

/// GET /api/carrera/leccion — protegida, requiere JWT
pub async fn leccion() -> Result<Json<LeccionResponse>, (StatusCode, String)> {
    match obtener_leccion_aleatoria().await {
        Ok(leccion) => Ok(Json(leccion)),
        Err(error) => {
            tracing::error!("No se pudo obtener la lección: {error}");
            Err((StatusCode::INTERNAL_SERVER_ERROR, "No se pudo cargar la lección".to_string()))
        }
    }
}

pub fn router_leccion() -> Router {
    Router::new().route("/api/leccion", get(leccion))
}