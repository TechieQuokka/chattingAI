use anyhow::Result;
use futures::Stream;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::pin::Pin;

#[derive(Debug, Clone, Serialize)]
pub struct OllamaChatRequest {
    pub model: String,
    pub messages: Vec<OllamaMessage>,
    pub stream: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<OllamaOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub think: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OllamaMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct OllamaOptions {
    pub num_ctx: u32,
}

#[derive(Debug, Deserialize)]
pub struct OllamaChatChunk {
    pub message: Option<OllamaMessage>,
    pub done: bool,
}

#[derive(Debug, Serialize)]
pub struct OllamaLoadRequest {
    pub model: String,
    pub keep_alive: String,
}

#[derive(Debug, Serialize)]
pub struct OllamaUnloadRequest {
    pub model: String,
    pub keep_alive: i32,
}

#[derive(Debug, Deserialize)]
pub struct OllamaModel {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct OllamaModelsResponse {
    pub models: Vec<OllamaModel>,
}

pub struct OllamaClient {
    pub client: Client,
    pub base_url: String,
}

impl OllamaClient {
    pub fn new(base_url: String) -> Self {
        Self {
            client: Client::builder()
                .timeout(std::time::Duration::from_secs(300))
                .build()
                .expect("Failed to build reqwest client"),
            base_url,
        }
    }

    pub async fn list_models(&self) -> Result<Vec<String>> {
        let url = format!("{}/api/tags", self.base_url);
        let resp = self.client.get(&url).send().await?;
        let data: OllamaModelsResponse = resp.json().await?;
        Ok(data.models.into_iter().map(|m| m.name).collect())
    }

    pub async fn load_model(&self, model: &str) -> Result<()> {
        let url = format!("{}/api/chat", self.base_url);
        let req = serde_json::json!({
            "model": model,
            "messages": [],
            "keep_alive": "10m"
        });
        self.client.post(&url).json(&req).send().await?;
        Ok(())
    }

    pub async fn unload_model(&self, model: &str) -> Result<()> {
        let url = format!("{}/api/chat", self.base_url);
        let req = serde_json::json!({
            "model": model,
            "messages": [],
            "keep_alive": 0
        });
        self.client.post(&url).json(&req).send().await?;
        Ok(())
    }

    pub async fn chat_stream(
        &self,
        model: &str,
        messages: Vec<OllamaMessage>,
        thinking: bool,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<String>> + Send>>> {
        let url = format!("{}/api/chat", self.base_url);
        let req = OllamaChatRequest {
            model: model.to_string(),
            messages,
            stream: true,
            options: Some(OllamaOptions { num_ctx: 8192 }),
            think: if thinking { Some(true) } else { None },
        };

        let resp = self.client.post(&url).json(&req).send().await?;
        let byte_stream = resp.bytes_stream();

        let stream = tokio_stream::StreamExt::map(byte_stream, |chunk| {
            let bytes = chunk.map_err(|e| anyhow::anyhow!(e))?;
            let text = String::from_utf8_lossy(&bytes).to_string();
            Ok(text)
        });

        Ok(Box::pin(stream))
    }
}

#[derive(Debug, Deserialize)]
pub struct OllamaShowResponse {
    pub capabilities: Option<Vec<String>>,
}

impl OllamaClient {
    pub async fn show_model(&self, model: &str) -> Result<Vec<String>> {
        let url = format!("{}/api/show", self.base_url);
        let req = serde_json::json!({ "model": model });
        let resp = self.client.post(&url).json(&req).send().await?;
        let data: OllamaShowResponse = resp.json().await?;
        Ok(data.capabilities.unwrap_or_default())
    }
}
