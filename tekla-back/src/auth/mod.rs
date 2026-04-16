// src/auth/mod.rs
pub mod jwt;
pub mod jwt_service;
pub mod middleware;
pub mod routes;

// Re-exportar tipos públicos para facilitar el acceso
pub use jwt::{Claims, UserRole, TokenPair, AuthResponse, AuthUser, JwtConfig};
pub use jwt_service::{JwtService, JwtError};
pub use middleware::{AuthMiddleware, AuthenticatedUser};
pub use routes::{auth_routes, LoginRequest, RefreshRequest, ApiResponse};