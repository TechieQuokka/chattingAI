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

/// Model capability flags — resolved from model name at load time
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelCaps {
    pub think: bool,
}

impl ModelCaps {
    pub fn none() -> Self {
        Self { think: false }
    }
}

/// Resolve capabilities from model name (prefix/substring matching)
pub fn resolve_caps(model: &str) -> ModelCaps {
    let m = model.to_lowercase();

    // gemma4 (all variants including e2b, e4b, latest, 27b ...)
    if m.contains("gemma4") || m.contains("gemma3n") {
        return ModelCaps { think: true };
    }
    // gemma3
    if m.contains("gemma3") {
        return ModelCaps { think: true };
    }
    // qwen3 / qwen3.5
    if m.contains("qwen3") {
        return ModelCaps { think: true };
    }
    // deepseek-r1
    if m.contains("deepseek-r1") {
        return ModelCaps { think: true };
    }
    // phi4-reasoning (must come before plain phi4)
    if m.contains("phi4-reasoning") {
        return ModelCaps { think: true };
    }
    if m.contains("phi4") {
        return ModelCaps { think: true };
    }
    // abliterated / custom gemma4 forks
    if m.contains("huihui") || m.contains("abliterated") || m.contains("fredrezones") {
        return ModelCaps { think: true };
    }

    // everything else: no think
    ModelCaps::none()
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
