// main.rs - SIN serde, SIN errores
use actix_cors::Cors;
use actix_web::{App, HttpResponse, HttpServer, Responder, get};
use std::sync::Mutex;
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

// Textos predefinidos - versión simple
const TEXTO_ESPANOL: [&str; 4] = [
    "La tecnología avanza a pasos agigantados, transformando cada aspecto de nuestra vida cotidiana.",
    "La inteligencia artificial y el aprendizaje automático están revolucionando industrias enteras, ofreciendo soluciones antes consideradas imposibles. Estas herramientas no solo optimizan tareas repetitivas, sino que también abren puertas a descubrimientos científicos.",
    "El desarrollo sostenible se ha convertido en una prioridad global ante los crecientes desafíos ambientales. La colaboración internacional y la innovación tecnológica son clave para encontrar soluciones.",
    "La educación del siglo XXI requiere una transformación profunda para preparar a las nuevas generaciones en un mundo en constante cambio. Las habilidades digitales y el pensamiento crítico son esenciales.",
];

// Endpoint GET principal
#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("API de Texto - Use /texto para obtener 2 párrafos")
}

#[get("/texto")]
async fn get_texto() -> impl Responder {
    let parrafo1 = TEXTO_ESPANOL[0];
    // let parrafo2 = TEXTO_ESPANOL[1];

    // Remover todos los saltos de línea y espacios múltiples
    let texto_limpio = format!(
        "{}",
        // parrafo1.replace("\n", " "), // Reemplaza saltos por espacio
        parrafo1.replace("\n", " ")
    );

    // También puedes remover espacios múltiples consecutivos
    let texto_final = texto_limpio
        .split_whitespace() // Divide por espacios
        .collect::<Vec<&str>>()
        .join(" "); // Vuelve a unir con un solo espacio

    // HttpResponse::Ok()
    //     .content_type("text/plain; charset=utf-8")
    //     .body(texto_final)

    HttpResponse::Ok().json(texto_final)
}

// Endpoint de salud
#[get("/salud")]
async fn salud() -> impl Responder {
    HttpResponse::Ok().body("🚀 API de Texto funcionando correctamente")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🚀 Iniciando API Rust con CORS en: http://127.0.0.1:8080");
    println!("🌐 Angular: http://localhost:4200");

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
