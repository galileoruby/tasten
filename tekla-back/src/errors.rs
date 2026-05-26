use axum::{http::StatusCode, response::{IntoResponse, Response}, Json};
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Credenciales inválidas")]
    InvalidCredentials,

    #[error("Token inválido o expirado")]
    InvalidToken,

    #[error("No autorizado")]
    Unauthorized,

    #[error("Error interno del servidor")]
    Internal(#[from] anyhow::Error),
}

/// Convierte AppError en una respuesta HTTP automáticamente
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            AppError::InvalidCredentials => (StatusCode::UNAUTHORIZED, self.to_string()),
            AppError::InvalidToken       => (StatusCode::UNAUTHORIZED, self.to_string()),
            AppError::Unauthorized       => (StatusCode::FORBIDDEN,    self.to_string()),
            AppError::Internal(_)        => (StatusCode::INTERNAL_SERVER_ERROR, "Error interno".into()),
        };

        (status, Json(json!({ "error": message }))).into_response()
    }
}