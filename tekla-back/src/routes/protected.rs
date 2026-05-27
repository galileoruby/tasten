use axum::{routing::get, Json, Router};
use serde_json::{json, Value};

use crate::{errors::AppError, middleware::auth::{AuthUser, RequireRole}};

/// GET /api/health — ruta pública, sin autenticación
async fn health() -> Json<Value> {
    Json(json!({
        "status": "ok",
        "version": env!("CARGO_PKG_VERSION")
    }))
}

/*
sugar syntax
async fn random(AuthUser(_claims):AuthUser) -> Json<Value> {
    let carrera = Carrera::leccion_aleatoria();
    Json(json!({
       "leccion": carrera
    }))
}
*/

fn random() -> impl std::future::Future<Output = Json<Value>> {
    async {
        let carrera = Carrera::leccion_aleatoria();
        Json(json!({"leccion": carrera}))
    }
}

/// GET /api/me — ruta protegida, cualquier usuario autenticado
async fn me(AuthUser(claims): AuthUser) -> Json<Value> {
    Json(json!({
        "username": claims.sub,
        "role":     claims.role,
        "exp":      claims.exp,
    }))
}

/// GET /api/admin — ruta protegida, solo rol "admin"
async fn admin_only(AuthUser(claims): AuthUser) -> Result<Json<Value>, AppError> {
    // Verificar rol dentro del handler
    let _ = RequireRole::check(claims.clone(), "admin")?;

    Ok(Json(json!({
        "message": "Bienvenido al panel de administración",
        "user":    claims.sub,
    })))
}

/// GET /api/dashboard — protegida, acepta admin o user
async fn dashboard(AuthUser(claims): AuthUser) -> Json<Value> {
    let content = match claims.role.as_str() {
        "admin" => "Vista completa del dashboard",
        _       => "Vista limitada del dashboard",
    };

    Json(json!({
        "content": content,
        "user":    claims.sub,
    }))
}

/// Registra todas las rutas de la API
pub fn router() -> Router {
    Router::new()
        .route("/api/health",    get(health))     // pública
        .route("/api/me",        get(me))          // 🔒 cualquier token válido
        .route("/api/admin",     get(admin_only))  // 🔒 solo admin
        .route("/api/dashboard", get(dashboard))   // 🔒 admin o user
        .route("/api/random", get(random)) //inicio de carrera
}
