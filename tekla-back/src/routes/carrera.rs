use axum::{routing::get,Json,Router};

use crate::{
    middleware::auth::AuthUser,
    models::carrera::LeccionResponse,
    services::carrera::obtener_leccion_aleatoria,
};

/// GET /api/carrera/leccion — protegida, requiere JWT
// pub async fn leccion(_auth: AuthUser) -> Json<LeccionResponse> {
pub async fn leccion() -> Json<LeccionResponse> {
    Json(obtener_leccion_aleatoria())
}

pub fn router_leccion() -> Router {
    Router::new().route("/api/leccion", get(leccion))
}