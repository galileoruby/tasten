// main.rs - SIN serde, SIN errores
use actix_cors::Cors;
use actix_web::{App, HttpResponse, HttpServer, Responder, get};
use std::sync::Mutex;

mod carrera;

use crate::carrera::Carrera;

// Estado compartido
struct AppState {
    visit_count: Mutex<u32>,
}


/*
    a.separar en clases
    b.generar textos aleatorios en cada solicitud
    c.almacenar en alguna nosql
    d.almacenar estadisticas recuperadas.
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
    println!("Iniciando API Rust con CORS en: http://127.0.0.1:8080");
    println!("Angular: http://localhost:4200");

    HttpServer::new(|| {
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
            .service(index)
            .service(salud)
            .service(get_texto)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
