use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, HeaderMap},
    RequestPartsExt,
};
use jsonwebtoken::{decode, DecodingKey, Validation};

use crate::{errors::AppError, models::users::Claims};

/// Extractor que valida el JWT y expone los Claims al handler
///
/// Uso en un handler:
///   async fn mi_ruta(claims: AuthUser) -> ...
pub struct AuthUser(pub Claims);

#[async_trait]
impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // 1. Extraer el header Authorization
        let headers: &HeaderMap = &parts.headers;

        let auth_header = headers
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .ok_or(AppError::Unauthorized)?;

        // 2. Verificar formato "Bearer <token>"
        let token = auth_header
            .strip_prefix("Bearer ")
            .ok_or(AppError::InvalidToken)?;

        // 3. Decodificar y validar el token
        let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "super_secret_dev".into());

        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(secret.as_bytes()),
            &Validation::default(), // valida firma + expiración automáticamente
        )
        .map_err(|_| AppError::InvalidToken)?;

        Ok(AuthUser(token_data.claims))
    }
}

/// Extractor adicional para rutas que requieren un rol específico
///
/// Uso en un handler:
///   async fn solo_admin(RequireRole(claims): RequireRole<"admin">) -> ...
pub struct RequireRole(pub Claims);

impl RequireRole {
    pub fn check(claims: Claims, required_role: &str) -> Result<Self, AppError> {
        if claims.role != required_role {
            return Err(AppError::Unauthorized);
        }
        Ok(RequireRole(claims))
    }
}