use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, errors::Error as JwtError, DecodingKey, EncodingKey, Header, Validation, Algorithm};
use serde::{Deserialize, Serialize};
use uuid::Uuid;


// 1. Claims (datos dentro del JWT)
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,           // Subject (usuario ID)
    pub email: String,         // Email del usuario
    pub role: UserRole,        // Rol del usuario
    pub exp: i64,              // Expiración (timestamp)
    pub iat: i64,              // Emitido en (timestamp)
    pub jti: String,           // JWT ID (para invalidación)
}

// 2. Roles de usuario (ejemplo)
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum UserRole {
    User,
    Admin,
    Moderator,
}


// 3. Token pair (access + refresh tokens)
#[derive(Debug, Serialize, Deserialize)]
pub struct TokenPair {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,    // "Bearer"
    pub expires_in: i64,       // Segundos hasta expiración
}

#[derive(Debug, Clone)]
pub struct JwtConfig {
    pub access_token_secret: String,
    pub refresh_token_secret: String,
    pub access_token_expiry: i64,    // en horas
    pub refresh_token_expiry: i64,   // en días
    pub issuer: String,
}

// 5. Respuesta de login exitoso
#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub tokens: TokenPair,
    pub user: AuthUser,
}

#[derive(Debug, Serialize, Clone)]  
pub struct AuthUser {
    pub id: String,
    pub email: String,
    pub role: UserRole,
}