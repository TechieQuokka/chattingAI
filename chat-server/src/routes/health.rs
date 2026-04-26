use axum::{extract::State, Json};
use serde_json::{json, Value};
use crate::state::AppState;

pub async fn health(State(state): State<AppState>) -> Json<Value> {
    let inner = state.inner.read().await;
    Json(json!({
        "status": "ok",
        "active_model": inner.active_model,
        "model_status": inner.model_status,
    }))
}
