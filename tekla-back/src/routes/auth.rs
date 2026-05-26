use axum::{routing::post, Json, Router};
use chrono::Utc;
use jsonwebtoken::{encode, EncodingKey, Header};

use crate::{
    errors::AppError,
    models::users::{Claims, LoginRequest, LoginResponse, User},
};

/// Base de datos simulada — reemplazar con tu DB real (SQLx, SeaORM, etc.)
fn mock_users() -> Vec<User> {
    vec![
        User {
            id: 1,
            username: "admin".into(),
            password_hash: "1234".into(), // ⚠️ En producción: hash con argon2/bcrypt
            role: "admin".into(),
        },
        User {
            id: 2,
            username: "user".into(),
            password_hash: "abcd".into(),
            role: "user".into(),
        },
    ]
}

/// POST /auth/login
async fn login(Json(payload): Json<LoginRequest>) -> Result<Json<LoginResponse>, AppError> {
    // 1. Buscar usuario
    let user = mock_users()
        .into_iter()
        .find(|u| u.username == payload.username)
        .ok_or(AppError::InvalidCredentials)?;

    // 2. Verificar contraseña (en producción: argon2::verify / bcrypt::verify)
    if user.password_hash != payload.password {
        return Err(AppError::InvalidCredentials);
    }

    // 3. Construir claims
    let now = Utc::now().timestamp() as usize;
    let exp = now + 3600; // Token válido por 1 hora

    let claims = Claims {
        sub: user.username.clone(),
        role: user.role.clone(),
        iat: now,
        exp,
    };

    // 4. Generar token
    let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "super_secret_dev".into());
    let token = encode(
        &Header::default(), // algoritmo: HS256
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|e| AppError::Internal(e.into()))?;

    Ok(Json(LoginResponse {
        token,
        token_type: "Bearer".into(),
    }))
}

/// Registra las rutas de autenticación
pub fn router() -> Router {
    Router::new().route("/auth/login", post(login))
}