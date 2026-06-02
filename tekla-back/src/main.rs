mod errors;
mod middleware;
mod models;
mod routes;
mod services;
mod state;

use axum::Router;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use state::AppState;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "backend=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app_state = AppState::new();

    // Router WS con su propio estado
    let ws_router = Router::new()
        .route("/ws/:room_id", axum::routing::get(routes::ws::ws_handler))
        .with_state(app_state);

    let app = Router::new()
        .merge(routes::auth::router())
        .merge(routes::protected::router())
        .merge(ws_router)
        .layer(cors)
        .layer(TraceLayer::new_for_http());

    let addr = "0.0.0.0:3000";
    tracing::info!("Servidor escuchando en {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}