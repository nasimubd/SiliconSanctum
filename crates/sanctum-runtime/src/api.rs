//! `OpenAI` and Anthropic compatible local gateway.
#![allow(clippy::missing_errors_doc)]

use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};
use reqwest::Client;
use serde_json::{Value, json};
use std::sync::Arc;

#[derive(Clone)]
pub struct ApiState {
    pub client: Client,
    pub upstream: String,
    pub default_model: String,
}

impl ApiState {
    #[must_use]
    pub fn new(upstream: impl Into<String>, default_model: impl Into<String>) -> Self {
        Self {
            client: Client::new(),
            upstream: upstream.into().trim_end_matches('/').to_owned(),
            default_model: default_model.into(),
        }
    }
}

pub fn router(state: ApiState) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/v1/models", get(models))
        .route("/v1/chat/completions", post(openai_chat))
        .route("/v1/responses", post(openai_chat))
        .route("/v1/completions", post(openai_chat))
        .route("/v1/messages", post(anthropic_messages))
        .with_state(Arc::new(state))
}

async fn health(State(state): State<Arc<ApiState>>) -> impl IntoResponse {
    let upstream = state
        .client
        .get(format!("{}/api/version", state.upstream))
        .send()
        .await;
    match upstream {
        Ok(response) if response.status().is_success() => (
            StatusCode::OK,
            Json(json!({
                "status": "ok",
                "upstream": "ready",
                "model": state.default_model,
            })),
        ),
        _ => (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(json!({
                "status": "degraded",
                "upstream": "unavailable",
                "model": state.default_model,
            })),
        ),
    }
}

async fn models(State(state): State<Arc<ApiState>>) -> impl IntoResponse {
    let response = state
        .client
        .get(format!("{}/api/tags", state.upstream))
        .send()
        .await;
    match response {
        Ok(response) => match response.json::<Value>().await {
            Ok(value) => {
                let models = value
                    .get("models")
                    .and_then(Value::as_array)
                    .cloned()
                    .unwrap_or_default()
                    .into_iter()
                    .map(|model| {
                        json!({
                            "id": model.get("name").and_then(Value::as_str).unwrap_or(&state.default_model),
                            "object": "model",
                            "owned_by": "local",
                        })
                    })
                    .collect::<Vec<_>>();
                (
                    StatusCode::OK,
                    Json(json!({"object":"list", "data":models})),
                )
            }
            Err(error) => upstream_error(error.to_string()),
        },
        Err(error) => upstream_error(error.to_string()),
    }
}

async fn openai_chat(
    State(state): State<Arc<ApiState>>,
    uri: axum::http::Uri,
    Json(request): Json<Value>,
) -> impl IntoResponse {
    let model = request
        .get("model")
        .and_then(Value::as_str)
        .unwrap_or(&state.default_model);
    let prompt = openai_prompt(&request);
    let ollama = json!({
        "model": model,
        "messages": [{"role":"user", "content": prompt}],
        "stream": false,
        "options": {"num_ctx": request.get("max_tokens").and_then(Value::as_u64).unwrap_or(32768)}
    });
    let response = state
        .client
        .post(format!("{}/api/chat", state.upstream))
        .json(&ollama)
        .send()
        .await;
    match response {
        Ok(response) => match response.json::<Value>().await {
            Ok(value) => {
                let content = value
                    .pointer("/message/content")
                    .and_then(Value::as_str)
                    .unwrap_or_default();
                let response = if uri.path().ends_with("/responses") {
                    json!({
                        "id":"resp_sanctum_local",
                        "object":"response",
                        "status":"completed",
                        "model":model,
                        "output":[{"type":"message","role":"assistant","content":[{"type":"output_text","text":content}]}],
                        "output_text":content,
                        "usage":{"input_tokens":value.get("prompt_eval_count").and_then(Value::as_u64).unwrap_or(0),"output_tokens":value.get("eval_count").and_then(Value::as_u64).unwrap_or(0)}
                    })
                } else {
                    json!({
                        "id":"sanctum-local",
                        "object":"chat.completion",
                        "model":model,
                        "choices":[{"index":0,"message":{"role":"assistant","content":content},"finish_reason":"stop"}],
                        "usage":{"prompt_tokens":value.get("prompt_eval_count").and_then(Value::as_u64).unwrap_or(0),"completion_tokens":value.get("eval_count").and_then(Value::as_u64).unwrap_or(0)}
                    })
                };
                (StatusCode::OK, Json(response))
            }
            Err(error) => upstream_error(error.to_string()),
        },
        Err(error) => upstream_error(error.to_string()),
    }
}

async fn anthropic_messages(
    State(state): State<Arc<ApiState>>,
    Json(request): Json<Value>,
) -> impl IntoResponse {
    let model = request
        .get("model")
        .and_then(Value::as_str)
        .unwrap_or(&state.default_model);
    let prompt = request
        .get("messages")
        .and_then(Value::as_array)
        .map(|messages| {
            messages
                .iter()
                .filter_map(|message| {
                    Some(format!(
                        "{}: {}",
                        message.get("role")?.as_str()?,
                        message.get("content")?.as_str()?
                    ))
                })
                .collect::<Vec<_>>()
                .join("\n")
        })
        .unwrap_or_default();
    let ollama =
        json!({"model":model,"messages":[{"role":"user","content":prompt}],"stream":false});
    let response = state
        .client
        .post(format!("{}/api/chat", state.upstream))
        .json(&ollama)
        .send()
        .await;
    match response {
        Ok(response) => match response.json::<Value>().await {
            Ok(value) => {
                let content = value
                    .pointer("/message/content")
                    .and_then(Value::as_str)
                    .unwrap_or_default();
                (
                    StatusCode::OK,
                    Json(json!({
                        "id":"msg_sanctum_local",
                        "type":"message",
                        "role":"assistant",
                        "model":model,
                        "content":[{"type":"text","text":content}],
                        "stop_reason":"end_turn",
                        "stop_sequence":null,
                        "usage":{"input_tokens":value.get("prompt_eval_count").and_then(Value::as_u64).unwrap_or(0),"output_tokens":value.get("eval_count").and_then(Value::as_u64).unwrap_or(0)}
                    })),
                )
            }
            Err(error) => upstream_error(error.to_string()),
        },
        Err(error) => upstream_error(error.to_string()),
    }
}

fn openai_prompt(request: &Value) -> String {
    if let Some(input) = request.get("input").and_then(Value::as_str) {
        return input.to_owned();
    }
    request
        .get("messages")
        .and_then(Value::as_array)
        .map(|messages| {
            messages
                .iter()
                .filter_map(|message| {
                    Some(format!(
                        "{}: {}",
                        message.get("role")?.as_str()?,
                        message.get("content")?.as_str()?
                    ))
                })
                .collect::<Vec<_>>()
                .join("\n")
        })
        .unwrap_or_default()
}

fn upstream_error(message: impl Into<String>) -> (StatusCode, Json<Value>) {
    let message = message.into();
    (
        StatusCode::BAD_GATEWAY,
        Json(json!({"error":{"message":message,"type":"upstream_error"}})),
    )
}
