// use crate::jwt::{Claims, JwtConfig, TokenPair, UserRole};

use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Algorithm, Validation, decode, encode};
use thiserror::Error;
use uuid::Uuid;
 

use super::jwt::{Claims, JwtConfig, TokenPair, UserRole};

#[derive(Debug, Error)]
pub enum JwtError {
    #[error("Token inválido: {0}")]
    InvalidToken(String),

    #[error("Token expirado")]
    TokenExpired,

    #[error("Error al generar token: {0}")]
    GenerationError(String),

    #[error("Error de validación: {0}")]
    ValidationError(String),
}

#[derive(Clone)]
pub struct JwtService {
    config: JwtConfig,
}


impl JwtService {
    pub fn new(config: JwtConfig) -> Self {
        Self { config }
    }
    
    // Método para crear tokens de acceso
    pub fn create_access_token(
        &self,
        user_id: &str,
        email: &str,
        role: UserRole,
    ) -> Result<String, JwtError> {
        let now = Utc::now();
        let expires_at = now + Duration::hours(self.config.access_token_expiry);
        
        let claims = Claims {
            sub: user_id.to_string(),
            email: email.to_string(),
            role,
            exp: expires_at.timestamp(),
            iat: now.timestamp(),
            jti: Uuid::new_v4().to_string(),
        };
        
        self.encode_token(&claims, &self.config.access_token_secret)
    }
    
    // Método para crear refresh token
    pub fn create_refresh_token(&self, user_id: &str) -> Result<String, JwtError> {
        let now = Utc::now();
        let expires_at = now + Duration::days(self.config.refresh_token_expiry);
        
        let claims = Claims {
            sub: user_id.to_string(),
            email: "".to_string(), // Refresh token no necesita email
            role: UserRole::User,  // Rol mínimo
            exp: expires_at.timestamp(),
            iat: now.timestamp(),
            jti: Uuid::new_v4().to_string(),
        };
        
        self.encode_token(&claims, &self.config.refresh_token_secret)
    }
    
    // Crear par de tokens (access + refresh)
    pub fn create_token_pair(
        &self,
        user_id: &str,
        email: &str,
        role: UserRole,
    ) -> Result<TokenPair, JwtError> {
        let access_token = self.create_access_token(user_id, email, role)?;
        let refresh_token = self.create_refresh_token(user_id)?;
        
        Ok(TokenPair {
            access_token,
            refresh_token,
            token_type: "Bearer".to_string(),
            expires_in: self.config.access_token_expiry * 3600, // horas a segundos
        })
    }
    
    // Validar access token
    pub fn validate_access_token(&self, token: &str) -> Result<Claims, JwtError> {
        self.decode_token(token, &self.config.access_token_secret)
    }
    
    // Validar refresh token
    pub fn validate_refresh_token(&self, token: &str) -> Result<Claims, JwtError> {
        self.decode_token(token, &self.config.refresh_token_secret)
    }
    
    // Refrescar tokens (usando refresh token válido)
    pub fn refresh_tokens(&self, refresh_token: &str) -> Result<TokenPair, JwtError> {
        let claims = self.validate_refresh_token(refresh_token)?;
        
        // Aquí deberías verificar en tu DB que el usuario aún existe
        // y obtener su email y rol actual
        // Por ahora, asumimos que tenemos la info en claims
        
        self.create_token_pair(&claims.sub, &claims.email, claims.role)
    }
    
    // Métodos privados auxiliares
    fn encode_token(&self, claims: &Claims, secret: &str) -> Result<String, JwtError> {
        let encoding_key = EncodingKey::from_secret(secret.as_bytes());
        
        encode(&Header::default(), claims, &encoding_key)
            .map_err(|e| JwtError::GenerationError(e.to_string()))
    }
    
    fn decode_token(&self, token: &str, secret: &str) -> Result<Claims, JwtError> {
        let decoding_key = DecodingKey::from_secret(secret.as_bytes());
        let validation = Validation::new(Algorithm::HS256);
        
        decode::<Claims>(token, &decoding_key, &validation)
            .map(|token_data| token_data.claims)
            .map_err(|e| match e.kind() {
                jsonwebtoken::errors::ErrorKind::ExpiredSignature => JwtError::TokenExpired,
                _ => JwtError::InvalidToken(e.to_string()),
            })
    }
}