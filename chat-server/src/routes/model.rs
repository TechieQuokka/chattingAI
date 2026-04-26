use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::ollama::OllamaClient;
use crate::state::{AppState, ModelStatus};

#[derive(Deserialize)]
pub struct ModelRequest {
    pub model: String,
}

#[derive(Serialize)]
pub struct ApiResponse {
    pub success: bool,
    pub message: String,
}

pub async fn list_models(State(state): State<AppState>) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let client = OllamaClient::new(state.ollama_base_url.clone());
    match client.list_models().await {
        Ok(models) => Ok(Json(json!({ "models": models }))),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": e.to_string() })),
        )),
    }
}

pub async fn model_status(State(state): State<AppState>) -> Json<Value> {
    let inner = state.inner.read().await;
    Json(json!({
        "active_model": inner.active_model,
        "status": inner.model_status,
        "thinking_enabled": inner.thinking_enabled,
        "supports_thinking": inner.model_caps.think,
    }))
}

pub async fn load_model(
    State(state): State<AppState>,
    Json(req): Json<ModelRequest>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    // Check if already loading
    {
        let inner = state.inner.read().await;
        if inner.model_status == ModelStatus::Loading {
            return Err((
                StatusCode::CONFLICT,
                Json(json!({ "error": "Model is already loading" })),
            ));
        }
        if inner.active_model.as_deref() == Some(&req.model)
            && inner.model_status == ModelStatus::Active
        {
            return Ok(Json(json!({ "success": true, "message": "Model already active", "supports_thinking": inner.model_caps.think })));
        }
    }

    // Unload existing if any
    {
        let existing = {
            let inner = state.inner.read().await;
            inner.active_model.clone()
        };
        if let Some(old_model) = existing {
            {
                let mut inner = state.inner.write().await;
                inner.model_status = ModelStatus::Unloading;
            }
            let client = OllamaClient::new(state.ollama_base_url.clone());
            let _ = client.unload_model(&old_model).await;
            {
                let mut inner = state.inner.write().await;
                inner.active_model = None;
                inner.history.clear();
                inner.model_status = ModelStatus::Unloaded;
            }
        }
    }

    // Resolve caps from Ollama API
    let client_for_caps = OllamaClient::new(state.ollama_base_url.clone());
    let capabilities = client_for_caps.show_model(&req.model).await.unwrap_or_default();
    let caps = crate::state::ModelCaps {
        think: capabilities.contains(&"thinking".to_string()),
    };

    // Set loading state
    {
        let mut inner = state.inner.write().await;
        inner.model_status = ModelStatus::Loading;
        inner.active_model = Some(req.model.clone());
        inner.model_caps = caps.clone();
        // thinking_enabled을 caps에 맞게 초기화
        inner.thinking_enabled = caps.think;
    }

    // Load model
    let client = OllamaClient::new(state.ollama_base_url.clone());
    match client.load_model(&req.model).await {
        Ok(_) => {
            let mut inner = state.inner.write().await;
            inner.model_status = ModelStatus::Active;
            Ok(Json(json!({
                "success": true,
                "message": format!("Model {} loaded", req.model),
                "supports_thinking": caps.think,
            })))
        }
        Err(e) => {
            let mut inner = state.inner.write().await;
            inner.active_model = None;
            inner.model_status = ModelStatus::Unloaded;
            inner.model_caps = crate::state::ModelCaps::none();
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": e.to_string() })),
            ))
        }
    }
}

pub async fn unload_model(State(state): State<AppState>) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let model = {
        let inner = state.inner.read().await;
        inner.active_model.clone()
    };

    match model {
        None => Ok(Json(json!({ "success": true, "message": "No model loaded" }))),
        Some(m) => {
            {
                let mut inner = state.inner.write().await;
                inner.model_status = ModelStatus::Unloading;
            }
            let client = OllamaClient::new(state.ollama_base_url.clone());
            let _ = client.unload_model(&m).await;
            {
                let mut inner = state.inner.write().await;
                inner.active_model = None;
                inner.model_status = ModelStatus::Unloaded;
                inner.history.clear();
                inner.model_caps = crate::state::ModelCaps::none();
                inner.thinking_enabled = true;
            }
            Ok(Json(json!({ "success": true, "message": format!("Model {} unloaded", m) })))
        }
    }
}
