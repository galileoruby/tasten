// main.rs - SIN serde, SIN errores
use actix_cors::Cors;
use actix_web::{App, HttpResponse, HttpServer, Responder, get ,web};
use std::sync::Mutex;

mod carrera;

use crate::carrera::Carrera;
//use crate::auth::jwt::JwtConfig;

mod auth;
use dotenv::dotenv;
use std::env;

use crate::auth::jwt::JwtConfig;
use crate::auth::{AuthenticatedUser,  JwtService, auth_routes};

// Estado compartido
struct AppState {
    visit_count: Mutex<u32>,
}

fn load_jwt_config() -> JwtConfig {
    dotenv().ok(); // Cargar .env

    JwtConfig {
        access_token_secret: env::var("JWT_ACCESS_SECRET")
            .expect("JWT_ACCESS_SECRET debe estar configurado"),
        refresh_token_secret: env::var("JWT_REFRESH_SECRET")
            .expect("JWT_REFRESH_SECRET debe estar configurado"),
        access_token_expiry: env::var("JWT_ACCESS_EXPIRY_HOURS")
            .unwrap_or("24".to_string())
            .parse()
            .expect("JWT_ACCESS_EXPIRY_HOURS debe ser un número"),
        refresh_token_expiry: env::var("JWT_REFRESH_EXPIRY_DAYS")
            .unwrap_or("30".to_string())
            .parse()
            .expect("JWT_REFRESH_EXPIRY_DAYS debe ser un número"),
        issuer: env::var("JWT_ISSUER").unwrap_or_else(|_| "myapp".to_string()),
    }
}

// Endpoint protegido con JWT
#[get("/protegido")]
 

/*
    a.separar en clases
    b.generar textos aleatorios en cada solicitud
    c.almacenar en alguna nosql
    d.almacenar estadisticas recuperadas.
    e.emitir websockets
*/

// Endpoint GET principal
#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("API de Texto - Use /texto para obtener 2 párrafos")
}

#[get("/texto")]
async fn get_texto() -> impl Responder {
    let leccion_aleatoria = Carrera::leccion_aleatoria();

    HttpResponse::Ok().json(leccion_aleatoria)
}

// Endpoint de salud
#[get("/salud")]
async fn salud() -> impl Responder {
    HttpResponse::Ok().body("API de Texto funcionando correctamente")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Iniciando API Rust con JWT en: http://127.0.0.1:8080");
    println!("Endpoints disponibles:");
    println!("  GET  /           - Página principal");
    println!("  GET  /salud      - Health check");
    println!("  GET  /texto      - Obtener texto aleatorio (público)");
    println!("  GET  /protegido  - Ruta protegida con JWT");
    println!("  POST /auth/login - Login para obtener JWT");
    println!("  POST /auth/refresh - Refrescar tokens");
    println!("  GET  /auth/me    - Obtener info del usuario autenticado");
    println!("  GET  /auth/admin - Solo para administradores");

    // Cargar configuración JWT
    let jwt_config = load_jwt_config();
    let jwt_service = JwtService::new(jwt_config);

    HttpServer::new(move || {
        // Configurar CORS
        let cors = Cors::default()
            .allow_any_origin() // Permitir cualquier origen (en desarrollo)
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
            .allowed_headers(vec![
                actix_web::http::header::AUTHORIZATION,
                actix_web::http::header::ACCEPT,
                actix_web::http::header::CONTENT_TYPE,
            ])
            .max_age(3600); // Cache de preflight por 1 hora

        App::new()
            .wrap(cors) // Aplicar middleware CORS
            .app_data(web::Data::new(jwt_service.clone()))
            .service(index)
            .service(salud)
            .configure(auth_routes)
            .service(get_texto)
            //rutas protegidas
            .service(
                web::scope("/api")
                .wrap(crate::auth::middleware::AuthMiddleware)
                .service(get_protegido)
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
