// src/auth/routes.rs
use actix_web::{web, HttpResponse, Responder, post, get};
use serde::{Deserialize, Serialize};

use crate::auth::jwt::{AuthResponse, AuthUser, UserRole};
use crate::auth::jwt_service::JwtService;
use crate::auth::middleware::AuthenticatedUser;

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct RefreshRequest {
    pub refresh_token: String,
}

#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

type LoginResponse = ApiResponse<AuthResponse>;
// Tipo específico para respuestas de refresh
//type RefreshResponse = ApiResponse<TokenPair>;
// Tipo específico para respuestas de usuario
type UserResponse = ApiResponse<AuthUser>;
// Tipo específico para mensajes de texto
type MessageResponse = ApiResponse<String>;

#[post("/login")]
pub async fn login(
    jwt_service: web::Data<JwtService>,
    payload: web::Json<LoginRequest>,
) -> impl Responder {
    // Validación simple - en producción verificar contra DB
    if payload.email.is_empty() || payload.password.is_empty(){
        return HttpResponse::BadRequest().json(LoginResponse{
            success: false,
            data: None,
            error: Some("Email y contrseña son requeridos".to_string())
        });
    
  
    }
    
    // Usuario dummy
    let user_id = "12345";
    let role = if payload.email.contains("admin") {
        UserRole::Admin
    } else {
        UserRole::User
    };
    
    match jwt_service.create_token_pair(user_id, &payload.email, role) {
        Ok(tokens) => {
            HttpResponse::Ok().json(LoginResponse  {
                success: true,
                data: Some(AuthResponse {
                    tokens,
                    user: AuthUser {
                        id: user_id.to_string(),
                        email: payload.email.clone(),
                        role,
                    },
                }),
                error: None,
            })
        }
        Err(e) => {
            HttpResponse::InternalServerError().json(LoginResponse {
                success: false,
                data: None,
                error: Some(format!("Error al generar tokens: {}", e)),
            })
        }
    }
}

#[post("/refresh")]
pub async fn refresh_token(
    jwt_service: web::Data<JwtService>,
    payload: web::Json<RefreshRequest>,
) -> impl Responder {
    match jwt_service.refresh_tokens(&payload.refresh_token) {
        Ok(tokens) => {
            HttpResponse::Ok().json(ApiResponse {
                success: true,
                data: Some(tokens),
                error: None,
            })
        }
        Err(e) => {
            let status = match e {
                crate::auth::jwt_service::JwtError::TokenExpired => actix_web::http::StatusCode::UNAUTHORIZED,
                _ => actix_web::http::StatusCode::BAD_REQUEST,
            };
            
            HttpResponse::build(status).json(LoginResponse {
                success: false,
                data: None,
                error: Some(format!("Error al refrescar token: {}", e)),
            })
        }
    }
}


#[get("/me")]
pub async fn get_current_user(
    user: AuthenticatedUser,
) -> impl Responder {
    HttpResponse::Ok().json(UserResponse {
        success: true,
        data: Some(AuthUser {
            id: user.user_id,
            email: user.email,
            role: user.role,
        }),
        error: None,
    })
}

#[get("/admin")]
pub async fn admin_only(
    user: AuthenticatedUser,
) -> impl Responder {
    // Verificar si es admin
    if user.role != UserRole::Admin {
        return HttpResponse::Forbidden().json(UserResponse {
            success: false,
            data: None,
            error: Some("Permisos insuficientes".to_string()),
        });
    }
    
    HttpResponse::Ok().json(MessageResponse {
        success: true,
        data: Some(format!(
            "Hola admin {}, tienes acceso a esta área protegida",
            user.email
        )),
        error: None,
    })
}

// Función para configurar todas las rutas de auth
pub fn auth_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .service(login)
            .service(refresh_token)
            .service(get_current_user)
            .service(admin_only)
    );
}