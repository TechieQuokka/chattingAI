mod ollama;
mod routes;
mod state;

use axum::{
    routing::{delete, get, post},
    Router,
};
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::state::AppState;

#[tokio::main]
async fn main() {
    // Init logging
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "chat_server=info,axum=info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let ollama_url = std::env::var("OLLAMA_URL")
        .unwrap_or_else(|_| "http://localhost:11434".to_string());

    let port: u16 = std::env::var("PORT")
        .unwrap_or_else(|_| "3456".to_string())
        .parse()
        .unwrap_or(3456);

    let state = AppState::new(ollama_url.clone());
    let state_for_shutdown = state.clone();

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        // Health
        .route("/health", get(routes::health::health))
        // Models
        .route("/models", get(routes::model::list_models))
        .route("/model/load", post(routes::model::load_model))
        .route("/model/unload", post(routes::model::unload_model))
        .route("/model/status", get(routes::model::model_status))
        // Chat
        .route("/chat", post(routes::chat::chat))
        .route("/chat/history", delete(routes::chat::clear_history))
        .route("/chat/thinking", post(routes::chat::set_thinking))
        .layer(cors)
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    tracing::info!("chat-server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal(state_for_shutdown, ollama_url))
        .await
        .unwrap();
}

async fn shutdown_signal(state: AppState, ollama_url: String) {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("Failed to install SIGTERM handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    tracing::info!("Shutdown signal received. Unloading model...");

    // Unload active model before exit
    let model = {
        let inner = state.inner.read().await;
        inner.active_model.clone()
    };

    if let Some(model_name) = model {
        let client = ollama::OllamaClient::new(ollama_url);
        if let Err(e) = client.unload_model(&model_name).await {
            tracing::error!("Failed to unload model on shutdown: {}", e);
        } else {
            tracing::info!("Model {} unloaded successfully", model_name);
        }
    }

    tracing::info!("chat-server shutdown complete.");
}
