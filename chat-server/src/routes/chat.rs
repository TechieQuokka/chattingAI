use axum::{
    extract::State,
    http::StatusCode,
    response::Sse,
    Json,
};
use axum::response::sse::Event;
use futures::{Stream, StreamExt};
use serde::Deserialize;
use serde_json::{json, Value};
use std::convert::Infallible;

use crate::ollama::{OllamaClient, OllamaMessage};
use crate::state::{AppState, Message, ModelStatus};

#[derive(Deserialize)]
pub struct ChatRequest {
    pub message: String,
    pub thinking: Option<bool>,
}

pub async fn chat(
    State(state): State<AppState>,
    Json(req): Json<ChatRequest>,
) -> Result<Sse<impl Stream<Item = Result<Event, Infallible>>>, (StatusCode, Json<Value>)> {
    // Check model is active
    let (model, thinking_enabled) = {
        let inner = state.inner.read().await;
        if inner.model_status != ModelStatus::Active {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(json!({ "error": "No model is active. Load a model first." })),
            ));
        }
        let model = inner.active_model.clone().unwrap();
        let thinking = req.thinking.unwrap_or(inner.thinking_enabled);
        (model, thinking)
    };

    // Add user message to history
    {
        let mut inner = state.inner.write().await;
        inner.history.push(Message {
            role: "user".to_string(),
            content: req.message.clone(),
        });
    }

    // Build ollama messages from history
    let ollama_messages: Vec<OllamaMessage> = {
        let inner = state.inner.read().await;
        inner
            .history
            .iter()
            .map(|m| OllamaMessage {
                role: m.role.clone(),
                content: m.content.clone(),
            })
            .collect()
    };

    // Start streaming from ollama
    let client = OllamaClient::new(state.ollama_base_url.clone());
    let byte_stream = match client.chat_stream(&model, ollama_messages, thinking_enabled).await {
        Ok(s) => s,
        Err(e) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": e.to_string() })),
            ));
        }
    };

    // SSE stream: parse thinking vs answer
    let state_clone = state.clone();
    let sse_stream = async_stream::stream! {
        let mut full_response = String::new();
        let mut in_think_block = false;
        let mut think_buffer = String::new();
        let mut answer_buffer = String::new();
        let mut leftover = String::new();

        tokio::pin!(byte_stream);

        while let Some(chunk_result) = byte_stream.next().await {
            let raw = match chunk_result {
                Ok(r) => r,
                Err(e) => {
                    let ev = Event::default()
                        .event("error")
                        .data(serde_json::json!({ "error": e.to_string() }).to_string());
                    yield Ok(ev);
                    break;
                }
            };

            // Ollama streams newline-delimited JSON
            let combined = format!("{}{}", leftover, raw);
            let mut lines = combined.split('\n').collect::<Vec<_>>();
            // Keep last (possibly incomplete) line as leftover
            if combined.ends_with('\n') {
                leftover = String::new();
            } else {
                leftover = lines.pop().unwrap_or("").to_string();
            }

            for line in lines {
                let line = line.trim();
                if line.is_empty() { continue; }
                let parsed: serde_json::Value = match serde_json::from_str(line) {
                    Ok(v) => v,
                    Err(_) => continue,
                };

                // Check for thinking field (gemma4 style)
                if let Some(thinking_content) = parsed["message"]["thinking"].as_str() {
                    if !thinking_content.is_empty() {
                        let ev = Event::default()
                            .event("thinking")
                            .data(serde_json::json!({ "chunk": thinking_content }).to_string());
                        yield Ok(ev);
                        think_buffer.push_str(thinking_content);
                    }
                }

                // Parse content for <|think|> tags (fallback for models that embed thinking in content)
                if let Some(content) = parsed["message"]["content"].as_str() {
                    let mut remaining = content;

                    while !remaining.is_empty() {
                        if in_think_block {
                            if let Some(end_pos) = remaining.find("<|/think|>") {
                                let think_part = &remaining[..end_pos];
                                if !think_part.is_empty() {
                                    think_buffer.push_str(think_part);
                                    let ev = Event::default()
                                        .event("thinking")
                                        .data(serde_json::json!({ "chunk": think_part }).to_string());
                                    yield Ok(ev);
                                }
                                in_think_block = false;
                                remaining = &remaining[end_pos + "<|/think|>".len()..];
                            } else {
                                // Still in think block
                                think_buffer.push_str(remaining);
                                let ev = Event::default()
                                    .event("thinking")
                                    .data(serde_json::json!({ "chunk": remaining }).to_string());
                                yield Ok(ev);
                                remaining = "";
                            }
                        } else {
                            if let Some(start_pos) = remaining.find("<|think|>") {
                                // Text before think block
                                let before = &remaining[..start_pos];
                                if !before.is_empty() {
                                    answer_buffer.push_str(before);
                                    full_response.push_str(before);
                                    let ev = Event::default()
                                        .event("answer")
                                        .data(serde_json::json!({ "chunk": before }).to_string());
                                    yield Ok(ev);
                                }
                                in_think_block = true;
                                remaining = &remaining[start_pos + "<|think|>".len()..];
                            } else {
                                // Normal answer content
                                answer_buffer.push_str(remaining);
                                full_response.push_str(remaining);
                                let ev = Event::default()
                                    .event("answer")
                                    .data(serde_json::json!({ "chunk": remaining }).to_string());
                                yield Ok(ev);
                                remaining = "";
                            }
                        }
                    }
                }

                // Check if done
                if parsed["done"].as_bool() == Some(true) {
                    // Save assistant message to history
                    let final_response = full_response.clone();
                    let mut inner = state_clone.inner.write().await;
                    inner.history.push(Message {
                        role: "assistant".to_string(),
                        content: final_response,
                    });

                    let ev = Event::default()
                        .event("done")
                        .data("{}");
                    yield Ok(ev);
                    return;
                }
            }
        }

        // Final done if stream ended without explicit done
        let final_response = full_response.clone();
        let mut inner = state_clone.inner.write().await;
        if !final_response.is_empty() {
            inner.history.push(Message {
                role: "assistant".to_string(),
                content: final_response,
            });
        }
        let ev = Event::default().event("done").data("{}");
        yield Ok(ev);
    };

    Ok(Sse::new(sse_stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(std::time::Duration::from_secs(15))
            .text("keep-alive"),
    ))
}

pub async fn clear_history(State(state): State<AppState>) -> Json<Value> {
    let mut inner = state.inner.write().await;
    inner.history.clear();
    Json(json!({ "success": true, "message": "History cleared" }))
}

pub async fn set_thinking(
    State(state): State<AppState>,
    Json(body): Json<serde_json::Value>,
) -> Json<Value> {
    let enabled = body["enabled"].as_bool().unwrap_or(true);
    let mut inner = state.inner.write().await;
    inner.thinking_enabled = enabled;
    Json(json!({ "success": true, "thinking_enabled": enabled }))
}
