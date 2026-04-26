use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ModelStatus {
    Unloaded,
    Loading,
    Active,
    Unloading,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}

/// Model capability flags — resolved from Ollama API at load time
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelCaps {
    pub think: bool,
}

impl ModelCaps {
    pub fn none() -> Self {
        Self { think: false }
    }
}

#[derive(Debug)]
pub struct InnerState {
    pub active_model: Option<String>,
    pub model_status: ModelStatus,
    pub history: Vec<Message>,
    pub thinking_enabled: bool,
    pub model_caps: ModelCaps,
}

impl InnerState {
    pub fn new() -> Self {
        Self {
            active_model: None,
            model_status: ModelStatus::Unloaded,
            history: Vec::new(),
            thinking_enabled: true,
            model_caps: ModelCaps::none(),
        }
    }
}

#[derive(Clone)]
pub struct AppState {
    pub inner: Arc<RwLock<InnerState>>,
    pub ollama_base_url: String,
}

impl AppState {
    pub fn new(ollama_base_url: String) -> Self {
        Self {
            inner: Arc::new(RwLock::new(InnerState::new())),
            ollama_base_url,
        }
    }
}
