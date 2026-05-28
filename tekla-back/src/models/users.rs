use serde::{Deserialize, Serialize};

/// Representa un usuario en el sistema
#[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct User {
        pub id: u32,
        pub username: String,
        pub password_hash: String, // En producción: usar bcrypt/argon2
        pub role: String,
    }

/// Payload que el cliente envía para hacer login
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

/// Respuesta al login exitoso
#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub token_type: String,
}

/// Claims que van dentro del JWT
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,   // subject (username o user_id)
    pub role: String,
    pub exp: usize,    // expiración (Unix timestamp)
    pub iat: usize,    // emitido en (Unix timestamp)
}